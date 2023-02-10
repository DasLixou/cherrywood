pub struct SystemResult {
    pub event_catched: bool,
}

impl SystemResult {
    pub const NEW: Self = SystemResult {
        event_catched: false,
    };
}
