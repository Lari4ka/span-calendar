use chrono::{Local, NaiveDate, TimeDelta};

use crate::{parse_date, time::Day};

#[derive(
    Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Span {
    pub(crate) id: Option<u64>,
    pub(crate) name: String,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
    pub(crate) duration: i64,
    pub(crate) created_by: u64,
}

impl<'a> FromIterator<&'a Span> for Vec<Span> {
    fn from_iter<T: for<'b> IntoIterator<Item = &'a Span>>(iter: T) -> Self {
        iter.into_iter()
            .map(|span| span.clone())
            .collect::<Vec<Span>>()
    }
}

impl Span {
    fn get_dates(&self) -> (NaiveDate, NaiveDate) {
        (parse_date(&self.start_date), parse_date(&self.end_date))
    }
    pub fn get_days_vec(&self) -> Vec<Day> {
        let mut days = Vec::new();
        let one_time_delta = TimeDelta::new(86_400, 0).unwrap();
        let (start_date, end_date) = self.get_dates();

        for i in 0..(end_date - start_date).num_days() + 1 {
            days.push(Day {
                date: start_date + (one_time_delta * i as i32),
                passed: false,
                included_in: None,
            });
        }
        days.iter_mut()
            .filter(|day| day.date <= Local::now().date_naive())
            .for_each(|day| day.passed = true);
        days
    }
}
