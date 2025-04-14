use dioxus::html::completions::CompleteWithBraces::{button, div, input};
use dioxus::logger::tracing;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus::*;
use time::Date;
use time::macros;
use time::macros::date;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        reiden_app { expy: "Acheron" }
        add_span { }
    }
}

#[component]
fn reiden_app(expy: String) -> Element {
    rsx! {
        "expy: {expy}"
    }
}

#[component]
fn add_span() -> Element {
    let mut start_date = use_signal(|| String::new());
    let mut end_date = use_signal(|| String::new());
    rsx! {
        div { }
        input {
            value: "{start_date}",
            id: "add-span-start-date",
            type: "date",
            oninput: move |input| start_date.set(input.value()),
        }
        div { }
        input {
            value: "{end_date}",
            id: "add-span-end-date",
            type: "date",
            oninput: move |input| end_date.set(input.value()),
        }
        div { }
        textarea {
            value: "{start_date}, {end_date}",
            id: "date_display",
            resize: false,
        }
    }
}

#[component]
fn time_span(
    start: Date,
    end: Date,
) -> Element {
    todo!()
}

#[derive(Props, PartialEq, Clone)]
pub struct ReidenAppProps {
    expy: String,
}


