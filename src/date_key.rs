use std::{collections::BTreeMap, time::Duration};

use rust_extensions::date_time::{DateTimeAsMicroseconds, DateTimeStruct, TimeStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub struct DateHourKey(i64);

impl DateHourKey {
    pub fn new(now: DateTimeAsMicroseconds) -> Self {
        let itm: DateTimeStruct = now.into();

        Self::from_components(
            itm.year as i64,
            itm.month as i64,
            itm.day as i64,
            itm.time.hour as i64,
        )
    }

    fn from_components(year: i64, month: i64, day: i64, hour: i64) -> Self {
        Self(year * 1000000 + month * 10000 + day * 100 + hour)
    }

    pub fn get_value(&self) -> i64 {
        self.0
    }

    pub fn get_keys_to_request(
        mut from: DateTimeAsMicroseconds,
        to: DateTimeAsMicroseconds,
    ) -> BTreeMap<Self, ()> {
        let to_key = Self::new(to);
        let mut current = Self::new(from);

        let mut result = BTreeMap::new();
        while current <= to_key {
            result.insert(current, ());
            from = from.add(Duration::from_secs(60 * 60));
            current = Self::new(from);
        }

        result
    }

    pub fn parse_from_str(value: &str) -> Option<Self> {
        if value.len() != 10 {
            return None;
        }

        let year = value[0..4].parse::<i64>().ok()?;
        let month = value[4..6].parse::<i64>().ok()?;
        let day = value[6..8].parse::<i64>().ok()?;
        let hour = value[8..10].parse::<i64>().ok()?;
        Some(Self::from_components(year, month, day, hour))
    }
}

impl From<i64> for DateHourKey {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for DateHourKey {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl From<DateTimeAsMicroseconds> for DateHourKey {
    fn from(value: DateTimeAsMicroseconds) -> Self {
        Self::new(value)
    }
}

impl From<DateHourKey> for DateTimeAsMicroseconds {
    fn from(value: DateHourKey) -> Self {
        let year = value.0 / 1000000;
        let month = (value.0 % 1000000) / 10000;
        let day = (value.0 % 10000) / 100;
        let hour = value.0 % 100;

        let dt = DateTimeStruct {
            year: year as i32,
            month: month as u32,
            day: day as u32,
            time: TimeStruct {
                hour: hour as u32,
                min: 0,
                sec: 0,
                micros: 0,
            },
            dow: None,
        };

        dt.try_into().unwrap()
    }
}
