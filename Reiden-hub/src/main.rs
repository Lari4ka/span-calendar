pub mod app;

use dioxus::html::completions::CompleteWithBraces::{button, div, h1, input};
use dioxus::html::form::action;
use dioxus::launch;
use dioxus::logger::tracing;
use dioxus::logger::tracing::{event, info};
use dioxus::prelude::*;

use regex::Regex;

use serde::de::value::StringDeserializer;

use std::collections::HashMap;

use time::macros;
use time::macros::date;
use time::Date;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

    }
}

#[component]
fn Main() -> Element {
    rsx! {
        AddComponent {}
        SpansComponent {}
    }
}

#[component]
fn AddComponent() -> Element {
    rsx! {}
}

#[component]
fn SpansComponent() -> Element {
    rsx! {}
}


