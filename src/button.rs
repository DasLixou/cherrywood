use std::any::TypeId;

use crate::{
    app::App, event::Event, event_catcher::EventCatcher, holding_ptr::HoldingPtr,
    system_batch::SystemBatch, widget::Widget,
};

pub struct Button {
    pub on_click: EventCatcher,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: EventCatcher::new(),
        }
    }

    pub fn subscribe_event<E: Event + 'static, B: SystemBatch>(mut self, systems: B) -> Self {
        self.on_click.subscribe(TypeId::of::<E>(), systems);
        self
    }
}

impl Widget for Button {
    fn dispatch_event(&mut self, app: &mut App, t: TypeId, _ptr: HoldingPtr) -> Option<HoldingPtr> {
        self.on_click.run(t, app);
        None
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
