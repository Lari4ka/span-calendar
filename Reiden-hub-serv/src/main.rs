use axum::response::IntoResponse;
use axum::{
    Json, Router,
    routing::post,
};
use std::error::Error;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/get_spans", post(get_spans))
        .route("/add_span", post(add_span))
        .route("/log_in", post(log_in))
        .route("/sign_up", post(sign_up))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("span-calendar.net")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn sign_up(Json(user): Json<UserSQL>) -> impl IntoResponse {
    let connection = rusqlite::Connection::open("./users.db3").unwrap();
    let sql = format!("SELECT COUNT (*) FROM users WHERE name = \"{}\"", user.name);
    let mut statement = connection.prepare(&sql).unwrap();
    let query = statement.query_one([], |row| row.get(0));
    let result = match query {
        Err(_) => {
            -1
        },
        Ok(num) => num,
    };

    if result != 0 {
        return Json(-1);
    }

    let mut statement = connection.prepare("SELECT MAX(id) FROM users").unwrap();
    let query = statement.query_one([], |row| row.get(0));
    let id = match query {
        Err(rusqlite::Error::QueryReturnedMoreThanOneRow) => -1,
        Err(rusqlite::Error::QueryReturnedNoRows) => -1,
        Err(_) => -1,
        Ok(num) => num,
    };

    if id == -1 {
        return Json(-1);
    }

    let sql = r#"
INSERT INTO users (
    name,
    id,
    password
    )
VALUES (
    ?1,
    ?2,
    ?3
);"#;
    println!("SignUped: {:?}, id: {}", user, id);
    connection
        .execute(sql, [user.name, (id + 1).to_string(), user.password])
        .unwrap();
    Json(id)
}

async fn log_in(Json(user): Json<UserSQL>) -> impl IntoResponse {
    if !is_valid_login(&user.name, &user.password) {
        return Json(-1);
    } else {
        let connection = rusqlite::Connection::open("./users.db3").unwrap();
        let sql = format!("SELECT id FROM users WHERE name = \"{}\"", user.name);
        let mut statement = connection.prepare(&sql).unwrap();
        let query = statement.query_one([], |row| row.get(0));
        match query {
            Err(_) => return Json(-1),
            Ok(id) => return Json(id),
        }
    }
}

fn is_valid_login(name: &str, password: &str) -> bool {
    let connection = rusqlite::Connection::open("./users.db3").unwrap();

    let sql = format!(
        "
    SELECT password FROM users WHERE name = \"{}\"
    ",
        name
    );

    let mut statement = connection.prepare(&sql).unwrap();

    let query = statement.query_one([], |row| row.get(0));

    let password_from_sql: String;

    match query {
        Err(_) => return false,
        Ok(password) => password_from_sql = password,
    }

    password == password_from_sql
}

async fn add_span(Json(span): Json<Span>) -> impl IntoResponse {
    println!("add: {:?}", span);
    let connection = rusqlite::Connection::open("./spans.db3").unwrap();

    let mut stmt = connection.prepare("SELECT MAX(id) FROM spans").unwrap();

    // get id of last span in db
    let id: u32 = stmt.query_one([], |row| row.get(0)).unwrap();
    println!("id: {}", id);

    let sql = r#"
INSERT INTO spans (
    id,
    name,
    start_date,
    end_date,
    duration,
    created_by
)
VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5,
    ?6
)"#;

    connection
        .execute(
            sql,
            [
                &(id + 1).to_string(),
                &span.name,
                &span.start_date,
                &span.end_date,
                &span.duration.to_string(),
                &span.created_by.to_string()
            ],
        )
        .unwrap();
        println!("inserted");
    Json(id)
}

async fn get_spans(Json(user): Json<User>) -> impl IntoResponse {
    println!("query by: {:?}", user);
    let con = rusqlite::Connection::open("./spans.db3").unwrap();
    let sql = format!(
        "SELECT id, name, start_date, end_date, duration, created_by FROM spans WHERE created_by = \"{}\"",
        user.id
    );

    let mut stm = con.prepare(&sql).unwrap();
    // get span data from db
    let rows = stm
        .query_map([], |row| {
            Ok(Span {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                start_date: row.get(2).unwrap(),
                end_date: row.get(3).unwrap(),
                duration: row.get(4).unwrap(),
                created_by: row.get(5).unwrap(),
            })
        })
        .unwrap();
    let mut spans: Vec<Span> = Vec::new();

    for row in rows {
        let span = row.unwrap();
        spans.push(span);
    }
    println!("users spans: {:?}", spans);
    Json(spans)
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Span {
    id: Option<u64>,
    name: String,
    start_date: String,
    end_date: String,
    duration: i64,
    created_by: u64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    id: u64,
    anonymous: bool,
    name: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            anonymous: true,
            name: "guest".to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserSQL {
    name: String,
    password: String,
}
