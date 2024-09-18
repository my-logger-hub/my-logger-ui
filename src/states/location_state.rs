#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    Dashboard,
    Logs,
    SettingsIgnoreList,
    SettingsOneTimeIgnore,
}

impl LocationState {
    pub fn copy_state(&self) -> Self {
        *self
    }
}
