use std::error::Error;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/get_spans", get(get_spans));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let connection = rusqlite::Connection::open("Reiden-hub/data/spans.db")?;

    let sql = r#"(
        INSERT INTO spans (
        id,
        start_date,
        end_date,
        name
        )
        VALUES (
        1, 
        )
    )"#;

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_spans() -> impl IntoResponse {

    /*
    //let con = rusqlite::Connection::open("./data.db3").unwrap();

    //let mut stm = con.prepare("SELECT id, start_date, end_date FROM spans").unwrap();

    let rows = stm.query_map([], |row| {
        Ok(Span {
            id: row.get(0).unwrap(),
            start_date: row.get(1).unwrap(),
            end_date: row.get(2).unwrap(),
        })
    }).unwrap();
    let mut spans: Vec<Span> = Vec::new();

    for row in rows {
        let span = row.unwrap();
        spans.push(span);
    }
    Json(spans)

     */
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Span {
    id: u32,
    start_date: String,
    end_date: String,
}
