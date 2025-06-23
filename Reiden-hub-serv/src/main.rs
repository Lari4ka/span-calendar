use axum::http::{status, StatusCode};
use axum::response::IntoResponse;
use axum::{Json, Router, routing::{get, post}};
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

async fn add_span(Json(span): Json<SpanEntry>) -> impl IntoResponse {

    println!("here");
    println!("{:?}", span);

    let connection = rusqlite::Connection::open("./spans.db3").unwrap();

    println!("WHERE");

    let sql = r#"
INSERT INTO spans (
    id,
    name,
    start_date,
    end_date
)
VALUES (
    1,
    ?1,
    ?2,
    DATE('now')
)"#;

    println!("HERE");

    connection.execute(sql, [&span.start_date, &span.end_date]).unwrap();

    StatusCode::OK
}

async fn get_spans() -> impl IntoResponse {
    let con = rusqlite::Connection::open("./spans.db3").unwrap();

    let mut stm = con
        .prepare("SELECT id, start_date, end_date FROM spans")
        .unwrap();

    let rows = stm
        .query_map([], |row| {
            Ok(Span {
                id: row.get(0).unwrap(),
                start_date: row.get(1).unwrap(),
                end_date: row.get(2).unwrap(),
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
    id: u32,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SpanEntry {
    start_date: String,
    end_date: String,
}
