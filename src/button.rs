use crate::{event::Event, widget::Widget};

pub struct Button {
    pub on_click: Event,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: Event::new(),
        }
    }
}

impl Widget for Button {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
