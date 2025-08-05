use std::time::Duration;
use std::vec;

use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono::Days;
use chrono::Local;
use chrono::TimeDelta;
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
    //togle add span menu
    let toggle_add_form = use_signal(|| false);

    //span data containers
    let start_date = use_signal(String::new);
    let end_date = use_signal(String::new);
    let name = use_signal(String::new);

    //spans container
    let mut spans = use_signal(Vec::<Span>::new);
    let mut calendar = use_signal(|| Calendar::default());

    //fill spans container from db
    use_future(move || async move {
        let vec = get_spans().await;
        calendar.set(Calendar::new(&vec));
        spans.set(vec);
    });
    rsx! {
        //hr { class: "vertical_line" }
        CurrentTimeComponent {  }
        MenuComponent { toggle_add_form },

        if toggle_add_form() {
            AddSpanComponent { start_date, end_date, name, toggle_add_form, spans }
        }
        //Test { spans }
        SpansComponent { spans }
        CalendarComponent { calendar }
    }
}

#[component]
fn Test(spans: Signal<Vec<Span>>) -> Element {
    let thing = ReadableVecExt::iter(&spans);

    let other_thing: Vec<Span> = thing.map(|span| span.clone().to_owned()).collect();
    rsx! {
        div {
            "{other_thing[0]:?}"
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

                            if parse_date(&end_date()) < parse_date(&start_date()) {
                                info!("swapped dates");
                                let temp = end_date();
                                end_date.set(start_date());
                                start_date.set(temp);
                            }

                            let add_result = add_span(start_date(), end_date(), name()).await;

                            match add_result {
                                Some(span) => {
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
                if toggle_error() {
                    ErrorComponent {}
                }
            }
        }
    }
}

#[component]
fn SpansComponent(spans: Signal<Vec<Span>>) -> Element {
    //let val2 = &val[0];

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

#[component]
fn CalendarComponent(calendar: Signal<Calendar>) -> Element {
    let days: Vec<Day> = calendar().days;
    rsx! {
        for i in 0..days.len() / 7 {
            div {
                class: "week_container",
                for j in 0..7 {
                    if days[i * 7 + j].passed {
                        div {
                            class: "passed_day_container",
                            "{days[i * 7 + j].date}"
                        }
                    } else {
                        div {
                            class: "day_container",
                            "{days[i * 7 + j].date}"
                        }
                    }
                }
            }
        }
    }
}

async fn add_span(start_date: String, end_date: String, name: String) -> Option<Span> {
    let parsed_start = parse_date(&start_date);
    let parsed_end = parse_date(&end_date);

    let duration = (parsed_end - parsed_start).num_days().abs();

    if duration <= 1 {
        return None;
    }

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
    info!("eeeeeeeeee");
    //get all spans from db on first launch of page
    reqwest::get("http://127.0.0.1:8081/get_spans")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn parse_date(date_str: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => return date,
        Err(e) => {
            info!("error: {:?}", e);
            NaiveDate::parse_from_str("2000-10-10", "%Y-%m-%d").unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct Span {
    id: Option<u64>,
    name: String,
    start_date: String,
    end_date: String,
    duration: i64,
}

impl Span {
    fn get_dates(&self) -> (NaiveDate, NaiveDate) {
        (parse_date(&self.start_date), parse_date(&self.end_date))
    }
    fn get_days_vec(&self) -> Vec<Day> {
        let mut days = Vec::new();
        let one_time_delta = TimeDelta::new(86_400, 0).unwrap();
        let (start_date, end_date) = self.get_dates();

        for i in 0..(end_date - start_date).num_days() + 1 {
            days.push(Day {
                date: start_date + (one_time_delta * i as i32),
                passed: false,
            });
        }
        days.iter_mut()
            .filter(|day| day.date <= Local::now().date_naive())
            .for_each(|day| day.passed = true);
        days
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Calendar {
    days: Vec<Day>,
}

impl Calendar {
    fn default() -> Self {
        Self { days: Vec::new() }
    }

    fn new(spans: &Vec<Span>) -> Calendar {
        let mut days = Vec::new();

        for span in spans {
            days.extend_from_slice(&span.get_days_vec());
        }
        //fill gaps
        for i in 1..days.len() {
            if days[i].date != days[i - 1].date.checked_add_days(Days::new(1)).unwrap() {
                let gap_length = (days[i].date - days[i - 1].date).num_days() as i32;
                for j in 1..gap_length {
                    let day = Day {
                        date: days[i - 1]
                            .date
                            .checked_add_days(Days::new(j as u64))
                            .unwrap(),
                        passed: false,
                    };
                    //info!("DAY: {:?}", day);
                    days.insert(i - 1 + j as usize, day);
                }
            }
        }

        let month_code: Vec<i32> = vec![0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
        let leap_year_month_code: Vec<i32> = vec![0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];
        let weekdays = vec![6, 0, 1, 2, 3, 4, 5];
        let year = Local::now().year();
        let is_leap_year = (year % 4 == 0 || year % 400 == 0) && year % 100 != 0;
        //curent day of month
        let day0 = days.first().unwrap().date.day0() as i32 + 1;
        //current month
        let month0 = days.first().unwrap().date.month0() as usize;
        //formula to get weekday from date
        let first_day = if !is_leap_year {
            (day0
                + month_code[month0]
                + 5 * ((year - 1) % 4)
                + 4 * ((year - 1) % 100)
                + 6 * ((year - 1) % 400))
                % 7
        } else {
            (day0
                + leap_year_month_code[month0]
                + 5 * ((year - 1) % 4)
                + 4 * ((year - 1) % 100)
                + 6 * ((year - 1) % 400))
                % 7
        };

        //make first day Monday
        for _i in 0..weekdays[first_day as usize] {
            days.insert(
                0,
                Day {
                    date: days
                        .first()
                        .unwrap()
                        .date
                        .checked_sub_days(Days::new(1))
                        .unwrap(),
                    passed: false,
                },
            );
        }
        // make last day Sunday
        if days.len() % 7 != 0 {
            for _i in 0..days.len() % 7 + 1 {
                days.push(Day {
                    date: days
                        .last()
                        .unwrap()
                        .date
                        .checked_add_days(Days::new(1))
                        .unwrap(),
                    passed: false,
                });
            }
        }

        Self { days }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Day {
    date: NaiveDate,
    passed: bool,
}
