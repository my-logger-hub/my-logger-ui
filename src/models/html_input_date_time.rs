use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::TimeZone;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlInputDateTime(DateTimeAsMicroseconds);

impl HtmlInputDateTime {
    pub fn try_from_str(value: &str) -> Option<Self> {
        let dt = DateTimeAsMicroseconds::from_str(value)?;
        Some(Self(dt))
    }

    pub fn to_local_time(&self, time_zone: TimeZone) -> Self {
        if time_zone.is_utc_zero() {
            return Self(self.0);
        }

        Self(time_zone.to_local_time(self.0))
    }

    pub fn to_utc_time(&self, time_zone: TimeZone) -> Self {
        if time_zone.is_utc_zero() {
            return Self(self.0);
        }

        Self(time_zone.to_utc_time(self.0))
    }

    pub fn to_string(&self) -> String {
        let mut result = self.0.to_rfc3339();
        result.truncate(16);
        result
    }

    pub fn get_unix_microseconds(&self) -> i64 {
        self.0.unix_microseconds
    }
}

impl Into<DateTimeAsMicroseconds> for HtmlInputDateTime {
    fn into(self) -> DateTimeAsMicroseconds {
        self.0
    }
}

impl Into<DateTimeAsMicroseconds> for &'_ HtmlInputDateTime {
    fn into(self) -> DateTimeAsMicroseconds {
        self.0
    }
}

impl Into<HtmlInputDateTime> for DateTimeAsMicroseconds {
    fn into(self) -> HtmlInputDateTime {
        HtmlInputDateTime(self)
    }
}
/*
fn iso_date_time_to_local_time(src: String, time_zone: TimeZone) -> String {
    if time_zone.is_utc_zero() {
        return src;
    }

    let dt = DateTimeAsMicroseconds::from_str(src.as_str()).unwrap();
    let dt = time_zone.to_local_time(dt);
    dt.to_rfc3339()[..16].to_string()
}

fn iso_date_time_to_utc_time(src: String, time_zone: TimeZone) -> String {
    if time_zone.is_utc_zero() {
        return src;
    }

    let dt = DateTimeAsMicroseconds::from_str(src.as_str()).unwrap();
    let dt = time_zone.to_utc_time(dt);
    dt.to_rfc3339()[..16].to_string()
}
 */
