use axum::response::IntoResponse;
use axum::{
    Json, Router,
    routing::{get, post},
};
use std::error::Error;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/get_spans", get(get_spans))
        .route("/add_span", post(add_span))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn add_span(Json(span): Json<Span>) -> impl IntoResponse {
    let connection = rusqlite::Connection::open("./spans.db3").unwrap();

    let mut stmt = connection.prepare("SELECT MAX(id) FROM spans").unwrap();

    // get id of last span in db
    let id: u32 = stmt.query_one([], |row| row.get(0)).unwrap();

    let sql = r#"
INSERT INTO spans (
    id,
    name,
    start_date,
    end_date,
    duration
)
VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5
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
            ],
        )
        .unwrap();

    Json(id)
}

async fn get_spans() -> impl IntoResponse {
    let con = rusqlite::Connection::open("./spans.db3").unwrap();

    let mut stm = con
        .prepare("SELECT id, name, start_date, end_date, duration FROM spans")
        .unwrap();
    // get span data from db
    let rows = stm
        .query_map([], |row| {
            Ok(Span {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                start_date: row.get(2).unwrap(),
                end_date: row.get(3).unwrap(),
                duration: row.get(4).unwrap(),
            })
        })
        .unwrap();
    let mut spans: Vec<Span> = Vec::new();

    for row in rows {
        let span = row.unwrap();
        spans.push(span);
    }
    Json(spans)
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Span {
    id: Option<u64>,
    name: String,
    start_date: String,
    end_date: String,
    duration: i64,
}
