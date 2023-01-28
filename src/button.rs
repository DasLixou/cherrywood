use crate::event::Event;

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
