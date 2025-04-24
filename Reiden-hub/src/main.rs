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
        document::Stylesheet { href: CSS }
        Main {}
    }
}

#[component]
fn Main() -> Element {

    let mut toggle_add_form = use_signal(|| false);

    let start_date = use_signal(String::new);
    let end_date = use_signal(String::new);

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
                        "add",
                        onclick: move |_| async move {
                            create_span(start_date(), end_date()).await.unwrap();
                        }
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

#[server]
async fn create_span(start_date: String, end_date: String) -> Result<(), ServerFnError> {
    Ok(())
}


