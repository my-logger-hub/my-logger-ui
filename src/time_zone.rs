use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TimeZone(i64);

impl TimeZone {
    pub fn create_utc_zero() -> Self {
        TimeZone(0)
    }
    pub fn to_string(&self) -> String {
        if self.0 < 0 {
            format!("UTC+{}", -self.0 as f64 / 60.0)
        } else {
            format!("UTC{}", -self.0 as f64 / 60.0)
        }
    }

    pub fn to_time_zone_date_time(&self, mut dt: DateTimeAsMicroseconds) -> DateTimeAsMicroseconds {
        dt.add_minutes(-self.0);
        dt
    }
}

impl Into<TimeZone> for i64 {
    fn into(self) -> TimeZone {
        TimeZone(self)
    }
}

impl Into<i64> for TimeZone {
    fn into(self) -> i64 {
        self.0
    }
}
