pub mod app;
pub mod model;

use std::collections::HashMap;
use dioxus::html::completions::CompleteWithBraces::{button, div, h1, input};
use dioxus::html::form::action;
use dioxus::launch;
use dioxus::logger::tracing;
use dioxus::logger::tracing::{event, info};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde::de::value::StringDeserializer;
use time::macros;
use time::macros::date;
use time::Date;
use regex::Regex;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let regexp = Regex::new("https:(.*?)\\.jpeg")?;
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://civitai.com/api/v1/images?limit=1")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });
    let url =  match &*img_src.read_unchecked() {
        Some(url) => url.clone(),
        None => String::new(),
    };
    let regexed = match regexp.find(&url) {
        Some(regex) => regex.as_str().to_string(),
        None => "NONE".to_string(),
    };
    info!("sus: {}", regexed);

    rsx! {
        div {
            img { src: regexed }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ImageResponse {
    success: bool,
    count: i32,
    data: String,
}
