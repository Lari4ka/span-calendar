use dioxus::html::completions::CompleteWithBraces::{button, div, h1, input};
use dioxus::html::form::action;
use dioxus::launch;
use dioxus::logger::tracing;
use dioxus::logger::tracing::{event, info};
use dioxus::prelude::*;

use dioxus::html::semantics::encoding;
use std::collections::HashMap;

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
        Main {}
    }
}

#[component]
fn Main() -> Element {
    let mut toggle_add_form = use_signal(|| false);

    let start_date = use_signal(String::new);
    let end_date = use_signal(String::new);
    let mut spans = use_signal(Vec::<Span>::new);

    use_future(move || async move {
        for span in get_spans().await {
            spans.write().push(span);
        }
    });

    info!("spans: {:?}", spans);

    rsx! {
        MenuComponent { toggle_add_form },
        if toggle_add_form() {
            AddSpanComponent { start_date, end_date }
        } else {
            SpansComponent {}
        }
    }
}

#[component]
fn MenuComponent(toggle_add_form: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "Menu",
            "menu",
            button {
                class: "add_span_button",
                onclick: move |_| {
                    toggle_add_form.set(!toggle_add_form())
                },
                if toggle_add_form() {
                    "do not add span",
                } else {
                    "add span",
                }
            }
        }
    }
}

#[component]
fn AddSpanComponent(start_date: Signal<String>, end_date: Signal<String>) -> Element {
    rsx! {
        main {
            div {
                div {
                    input {
                        class: "input_container",
                        type: "text",
                        placeholder: "start date",
                        oninput: move |input| {
                            println!("there");
                            start_date.set(input.value());
                        },
                    }
                }
                div {
                    input {
                        class: "input_container",
                        type: "end date",
                        placeholder: "end date",
                        oninput: move |input| {
                            end_date.set(input.value());
                        },
                    }
                }
                div {
                    button {
                        class: "add_span_button",
                        onclick: move |_| async move {
                            add_span(start_date(), end_date()).await;
                        },
                        "add",
                    }
                }
            }
        }
    }
}

#[component]
fn SpansComponent() -> Element {
    rsx! {
        div {
            "spans"
        }
    }
}

async fn add_span(start_date: String, end_date: String) {
    info!("adding");
    let span_entry = SpanEntry {
        start_date,
        end_date,
    };

    reqwest::Client::new()
        .post("http://127.0.0.1:8081/add_span")
        .json(&span_entry)
        .send()
        .await
        .unwrap();
}

async fn get_spans() -> Vec<Span> {
    reqwest::get("http://127.0.0.1:8081/get_spans")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
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
