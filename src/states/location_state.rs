#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    Dashboard,
    Logs,
    Settings,
    IgnoreSingleEvents,
}

impl LocationState {
    pub fn copy_state(&self) -> Self {
        *self
    }

    pub fn is_settings(&self) -> bool {
        match self {
            Self::Settings => true,
            _ => false,
        }
    }

    pub fn is_logs(&self) -> bool {
        match self {
            Self::Logs => true,
            _ => false,
        }
    }

    pub fn is_dashboard(&self) -> bool {
        match self {
            Self::Dashboard => true,
            _ => false,
        }
    }

    pub fn is_ignore_single_events(&self) -> bool {
        match self {
            Self::IgnoreSingleEvents => true,
            _ => false,
        }
    }
}
