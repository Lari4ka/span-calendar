pub mod app;
pub mod model;

use dioxus::html::completions::CompleteWithBraces::{button, div, h1, input};
use dioxus::html::form::action;
use dioxus::launch;
use dioxus::logger::tracing;
use dioxus::logger::tracing::{event, info};
use dioxus::prelude::*;

use regex::Regex;

use serde::de::value::StringDeserializer;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use time::macros;
use time::macros::date;
use time::Date;
use crate::model::model::{Span, Spans};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {

    let mut spans = Spans::new();

    let example_duration = ("01-01-0001".to_string(), "02-02-0002".to_string());
    let example = Span::new(example_duration);
    let test = spans.add(example);

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            id: "test",
            for i in spans {
                "{i}"
            }
        }
    }
}
