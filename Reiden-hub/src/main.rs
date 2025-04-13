use dioxus::logger::tracing;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Reiden Hub" }
}

#[component]
fn reiden_app(props: ReidenAppProps) -> Element {
    tracing::info!("expy: {expy}");
    todo!()
}

#[derive(Props, PartialEq, Clone)]
struct ReidenAppProps {
    expy: String,
}


