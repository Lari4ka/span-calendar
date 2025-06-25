use dioxus::launch;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/main.css");

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
    
    let toggle_add_form = use_signal(|| false);

    let start_date = use_signal(String::new);
    let end_date = use_signal(String::new);
    let name = use_signal(String::new);
    let mut spans = use_signal(Vec::<Span>::new);

    use_future(move || async move {
        for span in get_spans().await {
            spans.write().push(span);
        }
    });

    rsx! {
        MenuComponent { toggle_add_form },
        if toggle_add_form() {
            AddSpanComponent { start_date, end_date, name, toggle_add_form }
        }
        SpansComponent { spans }
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
fn AddSpanComponent(
        start_date: Signal<String>,
        end_date: Signal<String>,
        name: Signal<String>,
        toggle_add_form: Signal<bool>
    ) -> Element {
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
                    input {
                        class: "input_container",
                        type: "name",
                        placeholder: "name",
                        oninput: move |input| {
                            name.set(input.value());
                        },
                    }
                }
                div {
                    button {
                        class: "add_span_button",
                        onclick: move |_| async move {
                            add_span(start_date(), end_date(), name()).await;
                            toggle_add_form.set(!toggle_add_form());
                        },
                        "add",
                    }
                }
            }
        }
    }
}

#[component]
fn SpansComponent(spans: Signal<Vec<Span>>) -> Element {
    rsx! {
        for span in spans() {
            div {
                class: "span-component",
                "start: {span.start_date}, end: {span.end_date}"
             }
        }
    }
}

async fn add_span(start_date: String, end_date: String, name: String) {

    let span = Span {
        id: None,
        name,
        start_date,
        end_date,
    };

    reqwest::Client::new()
        .post("http://127.0.0.1:8081/add_span")
        .json(&span)
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
    id: Option<u64>,
    name: String,
    start_date: String,
    end_date: String,
}