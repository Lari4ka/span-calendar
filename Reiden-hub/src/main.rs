pub mod logic;
pub mod span;
pub mod time;

use std::time::Duration;
use std::vec;

use chrono::naive::NaiveDate;
use chrono::Local;
use dioxus::launch;
use dioxus::prelude::*;

use crate::span::Span;
use crate::time::Calendar;
use crate::time::Day;

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
    //user
    let user = use_signal(logic::User::default);

    //togle add span menu
    let toggle_add_form = use_signal(|| false);

    //spans container
    let mut spans = use_signal(Vec::<Span>::new);
    let mut calendar = use_signal(|| Calendar::default());

    //fill spans container from db
    let _ = use_resource(move || async move {
        if !user().anonymous {
            let vec = logic::get_spans(user()).await;

            if !vec.is_empty() {
                calendar.set(Calendar::new(&vec));
                calendar.write().round_up();
                spans.set(vec);
            }
        }
    });

    rsx! {
        CurrentTimeComponent {  }
        if user().anonymous { LogInOrSignUp { user } }
        else {
            MenuComponent { toggle_add_form },

            if toggle_add_form() {
                AddSpanComponent { toggle_add_form, spans, user, calendar }
            }
            SpansComponent { spans }
            CalendarComponent { calendar }
        }
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
    let mut time = use_signal(String::new);
    let formatter = "%Y-%m-%d, %H:%M:%S, GMT%Z %A";

    // asynchronously update timer
    use_future(move || async move {
        loop {
            let formatted = format!("{}", Local::now().format(formatter));
            time.set(formatted);
            async_std::task::sleep(Duration::from_millis(450)).await;
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

#[component]
fn LogInOrSignUp(user: Signal<logic::User>) -> Element {
    let mut name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut toggle_log_in_error = use_signal(|| false);
    let mut toggle_SignUp_error = use_signal(|| false);
    rsx! {
        div {
            class: "user_inputs_container",
            input {
                class: "user_inputs_container",
                id: "name_input",
                type: "text",
                placeholder: "name",
                oninput: move |input| {
                    name.set(input.value());
                }
            }
            input {
                class: "user_inputs_container",
                id: "password_input",
                type: "text",
                placeholder: "password",
                oninput: move |input| {
                    password.set(input.value());
                }
            }
            button {
                class: "user_inputs_container",
                id: "log_in_button",
                onclick: move |_| async move {

                    let potential_user = logic::User::new(name(), password());

                    let log_in_result = logic::UserSQL::validate(logic::UserSQL::new(&potential_user)).await;

                    match log_in_result {
                        Some(id) => {
                            let mut deref = user.write();
                            deref.anonymous = false;
                            deref.name = potential_user.name;
                            deref.id = Some(id);
                        },

                        None => toggle_log_in_error.set(true),
                    }

                },
                "Log In",
            }
            button {
                class: "user_inputs_container",
                id: "sign_up_button",
                onclick: move |_| async move {

                    let potential_user = logic::User::new(name(), password());
                    let SignUp_result = logic::UserSQL::sign_up(logic::UserSQL::new(&potential_user)).await;

                    match SignUp_result {
                        Some(id) => {
                            let mut deref = user.write();
                            deref.id = Some(id as u64);
                            deref.anonymous = false;
                        },
                        None => toggle_SignUp_error.set(true),
                    }
                },
                "Sign Up",
            }

            if toggle_log_in_error() { LogInErrorComponent {} }
            if toggle_SignUp_error() { SignUpErrorComponent {} }
        }
    }
}

//add span menu
#[component]
fn AddSpanComponent(
    toggle_add_form: Signal<bool>,
    spans: Signal<Vec<Span>>,
    user: Signal<logic::User>,
    calendar: Signal<Calendar>,
) -> Element {
    let mut toggle_error = use_signal(|| false);
    let mut start_date = use_signal(String::new);
    let mut end_date = use_signal(String::new);
    let mut name = use_signal(String::new);

    rsx! {
        main {
            div {
                input {
                    class: "input_container",
                    type: "date",
                    placeholder: "start date",
                    oninput: move |input| {
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

                        if parse_date(&end_date()) < parse_date(&start_date()) {
                            let temp = end_date();
                            end_date.set(start_date());
                            start_date.set(temp);
                        }
                        let add_result = logic::add_span(start_date(), end_date(), name(), user()).await;

                        match add_result {
                            Some(span) => {
                                calendar.write().add_span(&span);
                                time::Calendar::mark_included(&mut calendar.write().days, &spans());
                                spans.write().push(span);
                                toggle_error.set(false);
                            }
                            None => {
                                toggle_error.set(true);
                                start_date.set(String::new());
                                end_date.set(String::new());
                            }
                        }

                        //turn off add span menu
                        toggle_add_form.set(false);
                    },
                    "add",
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
fn WeekdaysHeader() -> Element {
    rsx! {
        div {
            id: "weekday_header_container",
            div {
                class: "weekday_container",
                "Monday",
            }
            div {
                class: "weekday_container",
                "Tuesday",
            }
            div {
                class: "weekday_container",
                "Wednesday",
            }
            div {
                class: "weekday_container",
                "Thursday",
            }
            div {
                class: "weekday_container",
                "Friday",
            }
            div {
                class: "weekday_container",
                "Saturday",
            }
            div {
                class: "weekday_container",
                "Sunday",
            }
        }
    }
}

#[component]
fn CalendarComponent(calendar: Signal<Calendar>) -> Element {
    let days: Vec<Day> = calendar().days;
    rsx! {
        WeekdaysHeader {}
        for i in 0..days.len() / 7 {
            div {
                class: "week_container",
                for j in 0..7 {
                    if days[i * 7 + j].passed {
                        div {
                            class: "passed_day_container",
                            button {
                                class: "dropbutton",
                                "{days[i * 7 + j].date}",
                            }
                            div {
                                class: "dropdown_content",
                                if !days[i * 7 + j].included_in.is_none() {
                                        for span in days[i * 7 + j].included_in.as_ref().unwrap() {
                                            a {
                                                "{span.name}"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "day_container",
                            button {
                                class: "dropbutton",
                                "{days[i * 7 + j].date}",
                            }
                            div {
                                class: "dropdown_content",
                                if !days[i * 7 + j].included_in.is_none() {
                                        for span in days[i * 7 + j].included_in.as_ref().unwrap() {
                                            a {
                                                "{span.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpanErrorComponent() -> Element {
    rsx! {
        div {
            id: "span_error_component",
            "span must be longer than one day"
        }
    }
}
#[component]
fn SignUpErrorComponent() -> Element {
    rsx! {
        div {
            id: "SignUp_error_component",
            "SignUp error"
        }
    }
}

#[component]
fn LogInErrorComponent() -> Element {
    rsx! {
        div {
            id: "log_in_error_component",
            "Log in error",
        }
    }
}

fn parse_date(date_str: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => return date,
        Err(_) => Local::now().date_naive(),
    }
}
