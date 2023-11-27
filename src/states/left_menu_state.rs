pub enum LeftMenuState {
    Dashboard,
    Logs,
}

impl LeftMenuState {
    pub fn new() -> Self {
        LeftMenuState::Logs
    }
}
