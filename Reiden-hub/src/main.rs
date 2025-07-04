use std::time::Duration;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Local;
use dioxus::launch;
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
    //togle add span menu
    let toggle_add_form = use_signal(|| false);

    //span data containers
    let start_date = use_signal(String::new);
    let end_date = use_signal(String::new);
    let name = use_signal(String::new);

    //spans container
    let mut spans = use_signal(Vec::<Span>::new);

    //fill spans container from db
    use_future(move || async move {
        for span in get_spans().await {
            spans.write().push(span);
        }
    });

    rsx! {
        hr { class: "vertical_line" }
        CurrentTimeComponent {  }
        MenuComponent { toggle_add_form },
        if toggle_add_form() {
            AddSpanComponent { start_date, end_date, name, toggle_add_form, spans }
        }
        SpansComponent { spans }
    }
}

//main menu
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

//current time component
#[component]
fn CurrentTimeComponent() -> Element {
    //timer
    let mut time = use_signal(|| Local::now());
    // asynchronously update timer
    use_future(move || async move {
        loop {
            time.set(Local::now());
            async_std::task::sleep(Duration::from_millis(1)).await;
        }
    });
    //render timer
    rsx! {
        div {
            class: "time_container",
            h1 { "time: {time.read()}" }
        }
    }
}

//add span menu
#[component]
fn AddSpanComponent(
    start_date: Signal<String>,
    end_date: Signal<String>,
    name: Signal<String>,
    toggle_add_form: Signal<bool>,
    spans: Signal<Vec<Span>>,
) -> Element {

    let mut toggle_error = use_signal(|| false);

    rsx! {
        main {
            div {
                div {
                    input {
                        class: "input_container",
                        type: "date",
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
                        type: "date",
                        placeholder: "end date",
                        oninput: move |input| {
                            end_date.set(input.value());
                        },
                    }
                }
                div {
                    input {
                        class: "input_container",
                        type: "text",
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

                            let add_result = add_span(start_date(), end_date(), name()).await;
                            
                            match add_result {
                                Some(span) => {
                                    spans.push(span);
                                    toggle_error.set(false);
                                }
                                None => {
                                    toggle_error.set(true);
                                    start_date.set(String::new());
                                    end_date.set(String::new());
                                }
                            }

                            //turn off add span menu
                            toggle_add_form.set(!toggle_add_form());
                        },
                        "add",
                    }
                }
                if toggle_error() {
                    ErrorComponent {}
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
                h3 {
                    class: "start_date_container",
                    "start date: {span.start_date}",
                }
                h3 {
                    class: "duration_container",
                    "duration: {span.duration} days"
                }
                h3 {
                    class: "end_date_container",
                    "end date: {span.end_date}",
                }
            }
        }
    }
}

#[component]
fn ErrorComponent() -> Element {
    rsx! {
        div { 
            class: "error_component",
            "Span must be longer than 1 day"
        }
    }
}

async fn add_span(start_date: String, end_date: String, name: String) -> Option<Span> {

    let parsed_start = parse_date(&start_date);
    let parsed_end = parse_date(&end_date);

    let duration = (parsed_end - parsed_start).num_days().abs();

    if duration <= 1 { return None }

    let mut span = Span {
        id: None,
        name,
        start_date,
        end_date,
        duration,
    };

    let id = send_to_server(&span).await;

    span.id = Some(id);
    Some(span)
}

async fn send_to_server(span: &Span) -> u64 {

    let client = reqwest::Client::new();

    let id: u64 = client
        .post("http://127.0.0.1:8081/add_span")
        .json(&span)
        .send()
        .await
        // request to add span to db
        .unwrap()
        .json()
        .await
        .unwrap();
        // get id of added span as a response

    id

}

async fn get_spans() -> Vec<Span> {
    //get all spans from db on first launch of page
    reqwest::get("http://127.0.0.1:8081/get_spans")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn parse_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap()
}

fn elapsed(span : Span) -> i64 {
    
    let start_date = parse_date(&span.start_date);
    let end_date = parse_date(&span.end_date);

    let duration = (end_date - start_date).num_days().abs();
    let now = Local::now().date_naive();
    
    let elapsed = if start_date > end_date { 
        (now - end_date).num_days() 
    } else {
        (now - start_date).num_days()
    };

    elapsed / duration

}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Span {
    id: Option<u64>,
    name: String,
    start_date: String,
    end_date: String,
    duration: i64,
}
