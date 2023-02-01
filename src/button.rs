use crate::{event_container::EventContainer, system_batch::SystemBatch, widget::Widget};

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
        _t: std::any::TypeId,
        _ptr: crate::holding_ptr::HoldingPtr,
    ) -> Option<crate::holding_ptr::HoldingPtr> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
