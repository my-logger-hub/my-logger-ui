use crate::models::*;

pub const TIME_RANGE_KEY: &str = "time-range";

pub fn get() -> TimeRange {
    let value = dioxus_utils::js::GlobalAppSettings::get_local_storage().get(TIME_RANGE_KEY);

    if value.is_none() {
        return TimeRange::default();
    }

    let value = value.unwrap();

    TimeRange::from_str(&value)
}

pub fn set(time_range: &TimeRange) {
    let value = time_range.to_string(TimeZone::create_utc_zero());

    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(TIME_RANGE_KEY, value.as_str());
}

pub fn set_as_str(time_range: &str) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(TIME_RANGE_KEY, time_range);
}

pub fn clear() {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().delete(TIME_RANGE_KEY);
}
