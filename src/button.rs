use crate::system::{DescribedSystem, IntoDescribedSystem};

pub struct Button {
    pub(crate) on_click: DescribedSystem,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: ().into_described(),
        }
    }

    pub fn on_click(mut self, callback: impl IntoDescribedSystem) -> Self {
        self.on_click = callback.into_described();
        self
    }
}
