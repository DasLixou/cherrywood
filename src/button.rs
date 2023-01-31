use crate::{event::Event, system_batch::SystemBatch, widget::Widget};

pub struct Button {
    pub on_click: Event,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: Event::new(),
        }
    }

    pub fn subscribe_on_click<B: SystemBatch>(mut self, systems: B) -> Self {
        self.on_click.subscribe(systems);
        self
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
