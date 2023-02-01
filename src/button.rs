use std::any::TypeId;

use crate::{
    app::App, event_container::EventContainer, holding_ptr::HoldingPtr, system_batch::SystemBatch,
    widget::Widget,
};

pub struct Button {
    pub on_click: EventContainer,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: EventContainer::new(),
        }
    }

    pub fn subscribe_on_click<B: SystemBatch>(mut self, systems: B) -> Self {
        self.on_click.subscribe(systems);
        self
    }
}

impl Widget for Button {
    fn dispatch_event(
        &mut self,
        app: &mut App,
        _t: TypeId,
        _ptr: HoldingPtr,
    ) -> Option<HoldingPtr> {
        self.on_click.run(app);
        None
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
