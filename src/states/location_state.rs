#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    Dashboard,
    Logs,
    Settings,
}

impl LocationState {
    pub fn copy_state(&self) -> Self {
        *self
    }
}
