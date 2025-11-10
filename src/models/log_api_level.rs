use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LogApiLevel {
    Info,
    Warning,
    Error,
    FatalError,
    Debug,
}

impl LogApiLevel {
    pub const ALL_LEVELS: [LogApiLevel; 5] = [
        LogApiLevel::Info,
        LogApiLevel::Warning,
        LogApiLevel::Error,
        LogApiLevel::FatalError,
        LogApiLevel::Debug,
    ];
    pub fn as_str(&self) -> &str {
        match self {
            LogApiLevel::Info => "Info",
            LogApiLevel::Warning => "Warning",
            LogApiLevel::Error => "Error",
            LogApiLevel::FatalError => "Fatal",
            LogApiLevel::Debug => "Debug",
        }
    }

    pub fn from_str(src: &str) -> LogApiLevel {
        match src {
            "Info" => LogApiLevel::Info,
            "Warning" => LogApiLevel::Warning,
            "Error" => LogApiLevel::Error,
            "Fatal" => LogApiLevel::FatalError,
            "Debug" => LogApiLevel::Debug,
            _ => LogApiLevel::Info,
        }
    }

    pub fn as_ref(&self) -> &LogApiLevel {
        self
    }
}
