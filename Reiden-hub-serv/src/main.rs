use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Json, Router,
    routing::{get, post},
};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

static mut CURRENT_ID: u64 = 1;

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

    let id = unsafe {
        let counter = Arc::new(Mutex::new(CURRENT_ID));
        let mut num = counter.lock().unwrap();
        *num += 1;
        CURRENT_ID = *num;
        *num
    };

    println!("id: {}", id);
    println!("SPAN JSON: {:?}", span);

    let connection = rusqlite::Connection::open("./spans.db3").unwrap();

    let sql = r#"
INSERT INTO spans (
    id,
    name,
    start_date,
    end_date
)
VALUES (
    ?1,
    ?2,
    ?3,
    ?4
)"#;

    connection
        .execute(
            sql,
            [
                &id.to_string(),
                &span.name,
                &span.start_date,
                &span.end_date,
            ],
        )
        .unwrap();

    StatusCode::OK
}

async fn get_spans() -> impl IntoResponse {
    let con = rusqlite::Connection::open("./spans.db3").unwrap();


    let mut stm = con
        .prepare("SELECT id, name, start_date, end_date FROM spans")
        .unwrap();

    let rows = stm
        .query_map([], |row| {
            Ok(Span {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                start_date: row.get(2).unwrap(),
                end_date: row.get(3).unwrap(),
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
}
