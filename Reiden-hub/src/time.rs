use chrono::{Datelike, Days, Local, NaiveDate};

use crate::{parse_date, span::Span};

#[derive(PartialEq, Clone, Debug)]
pub struct Calendar {
    pub(crate) days: Vec<Day>,
}

impl Calendar {
    pub fn default() -> Self {
        Self { days: Vec::new() }
    }

    pub fn new(spans: &Vec<Span>) -> Calendar {
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
                        included_in: None,
                    };
                    //info!("DAY: {:?}", day);
                    days.insert(i - 1 + j as usize, day);
                }
            }
        }

        days.sort();
        days.dedup();

        for day in days.iter_mut() {
            let spans = spans
                .iter()
                .filter(|span| {
                    day.date >= parse_date(&span.start_date)
                        && day.date <= parse_date(&span.end_date)
                })
                .collect::<Vec<Span>>();
            day.included_in = Some(spans);
        }

        Self { days }
    }

    pub fn round_up(&mut self) {
        let month_code: Vec<i32> = vec![0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
        let leap_year_month_code: Vec<i32> = vec![0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];
        let weekdays = vec![6, 0, 1, 2, 3, 4, 5];
        let year = Local::now().year();
        let is_leap_year = (year % 4 == 0 || year % 400 == 0) && year % 100 != 0;
        //curent day of month
        let day0 = self.days.first().unwrap().date.day0() as i32 + 1;
        //current month
        let month0 = self.days.first().unwrap().date.month0() as usize;
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
            self.days.insert(
                0,
                Day {
                    date: self
                        .days
                        .first()
                        .unwrap()
                        .date
                        .checked_sub_days(Days::new(1))
                        .unwrap(),
                    passed: false,
                    included_in: None,
                },
            );
        }
        // make last day Sunday
        if self.days.len() % 7 != 0 {
            for _i in 0..self.days.len() % 7 + 1 {
                self.days.push(Day {
                    date: self
                        .days
                        .last()
                        .unwrap()
                        .date
                        .checked_add_days(Days::new(1))
                        .unwrap(),
                    passed: false,
                    included_in: None,
                });
            }
        }
    }

    pub fn add_span(&mut self, span: &Span) {
        let days = span.get_days_vec();
        self.days.extend_from_slice(&days);
        self.days.sort();
        self.days.dedup();
        self.round_up();
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Day {
    pub(crate) date: NaiveDate,
    pub(crate) passed: bool,
    pub(crate) included_in: Option<Vec<Span>>,
}
