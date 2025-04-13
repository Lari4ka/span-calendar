use dioxus::html::completions::CompleteWithBraces::{button, div};
use dioxus::logger::tracing;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        reiden_app { expy: "Acheron" }
    }
}

#[component]
fn reiden_app(expy: String) -> Element {
    rsx! {
        div { "sus" }
        button {
            onclick: move |_| {
                info!("sussed")

            },
            "sus me",
        }
        "expy: {expy}"
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ReidenAppProps {
    expy: String,
}


