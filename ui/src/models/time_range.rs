use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::models::*;

use super::TimeZone;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeRange {
    HoursAgo(i32),
    Range(HtmlInputDateTime, HtmlInputDateTime),
    ExactHour(DateHourKey),
}

impl TimeRange {
    pub fn default() -> Self {
        TimeRange::HoursAgo(0)
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn to_string(&self, time_zone: TimeZone) -> String {
        match self {
            TimeRange::HoursAgo(v) => v.to_string(),
            TimeRange::Range(start, end) => format!(
                "{} - {}",
                start.to_local_time(time_zone).to_string(),
                end.to_local_time(time_zone).to_string(),
            ),

            TimeRange::ExactHour(k) => k.to_local_time(time_zone).to_string(),
        }
    }

    pub fn unwrap_as_range_mut(&mut self) -> (&mut HtmlInputDateTime, &mut HtmlInputDateTime) {
        match self {
            TimeRange::Range(start, end) => (start, end),
            _ => panic!("Not a range"),
        }
    }

    pub fn get_date_from_date_to(&self) -> (i64, i64) {
        match self {
            Self::HoursAgo(value) => (-(*value as i64), 0),
            Self::Range(from, to) => (from.get_unix_microseconds(), to.get_unix_microseconds()),
            Self::ExactHour(value) => {
                let dt: DateTimeAsMicroseconds = value.into();

                let result: DateHourKey = dt.into();
                (result.get_value(), 0)
            }
        }
    }

    pub fn from_str(value: &str) -> Self {
        //2024-09-17T22:19 - 2024-09-17T23:19
        if value.len() > 30 {
            if let Some(from_dt) = HtmlInputDateTime::try_from_str(&value[..16]) {
                if let Some(to_dt) = HtmlInputDateTime::try_from_str(&value[19..]) {
                    return TimeRange::Range(from_dt, to_dt);
                }
            }
        }

        let number = value.parse::<i32>();

        if let Ok(number) = number {
            return TimeRange::HoursAgo(number);
        }

        let date_hour_key = DateHourKey::try_from_str(value);

        if let Some(date_hour_key) = date_hour_key {
            return TimeRange::ExactHour(date_hour_key);
        }

        return TimeRange::default();
    }
}
