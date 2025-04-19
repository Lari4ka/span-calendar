use dioxus::hooks::use_resource;
use dioxus::prelude::{Readable, Resource};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Spans {
    pub spans: Vec<Span>,
}

impl Spans {
    pub fn new() -> Self {
        Spans { spans: Vec::new() }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Span {
    start: String,
    end: String,
}
