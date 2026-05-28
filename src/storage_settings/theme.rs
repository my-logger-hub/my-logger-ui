const LIGHT_VALUE: &str = "light";
const DARK_VALUE: &str = "dark";

pub const THEME_KEY: &str = "theme";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => LIGHT_VALUE,
            Self::Dark => DARK_VALUE,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            DARK_VALUE => Self::Dark,
            _ => Self::Light,
        }
    }

    pub fn toggled(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

pub fn get() -> Theme {
    let value = dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(THEME_KEY)
        .unwrap_or_default();

    Theme::from_str(&value)
}

pub fn set(theme: Theme) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(THEME_KEY, theme.as_str());
    apply(theme);
}

/// Writes `data-theme` onto the <html> element so the CSS token layer switches.
pub fn apply(theme: Theme) {
    dioxus_utils::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{}')",
        theme.as_str()
    ));
}

/// Reads persisted theme and applies it to <html>. Call once on app mount.
pub fn apply_current() {
    apply(get());
}
