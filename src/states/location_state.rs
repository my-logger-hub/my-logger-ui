#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    Dashboard,
    Logs,
    IgnoreList,
    OneTimeIgnore,
    Settings,
}

impl LocationState {
    pub fn copy_state(&self) -> Self {
        *self
    }
}
