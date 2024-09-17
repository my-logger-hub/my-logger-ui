use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{date_key::DateHourKey, dialogs::TimeRange};

pub const TIME_RANGE_KEY: &str = "time-range";

pub fn get() -> TimeRange {
    let value = dioxus_utils::js::GlobalAppSettings::get_local_storage().get(TIME_RANGE_KEY);

    if value.is_none() {
        return TimeRange::default();
    }

    let value = value.unwrap();

    //2024-09-17T22:19 - 2024-09-17T23:19
    if value.len() > 30 {
        let from = value[..16].to_string();
        let to = value[19..].to_string();

        let from_dt = DateTimeAsMicroseconds::from_str(from.as_str());
        let to_dt = DateTimeAsMicroseconds::from_str(to.as_str());

        if from_dt.is_some() && to_dt.is_some() {
            return TimeRange::Range(from, to);
        }
    }

    let number = value.as_str().parse::<i32>();

    if let Ok(number) = number {
        return TimeRange::HoursAgo(number);
    }

    let date_hour_key = DateHourKey::try_from_str(value.as_str());

    if let Some(date_hour_key) = date_hour_key {
        return TimeRange::ExactHour(date_hour_key);
    }

    return TimeRange::default();
}

pub fn save(time_range: &TimeRange) {
    let value = time_range.to_string();

    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(TIME_RANGE_KEY, value.as_str());
}
