use rest_api_shared::LogApiLevel;

pub trait LogApiLevelExt {
    const ALL_LEVELS: [LogApiLevel; 5];

    /// The reverse of `as_str()`. An unknown value is `Info`.
    fn from_str(src: &str) -> LogApiLevel;
}

impl LogApiLevelExt for LogApiLevel {
    const ALL_LEVELS: [LogApiLevel; 5] = [
        LogApiLevel::Info,
        LogApiLevel::Warning,
        LogApiLevel::Error,
        LogApiLevel::FatalError,
        LogApiLevel::Debug,
    ];

    fn from_str(src: &str) -> LogApiLevel {
        Self::ALL_LEVELS
            .into_iter()
            .find(|level| level.as_str() == src)
            .unwrap_or(LogApiLevel::Info)
    }
}
