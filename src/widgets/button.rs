use std::any::TypeId;

use crate::{
    event::EventMessage,
    event_rack::EventRack,
    system::BoxedDescribedSystem,
    system_batch::SystemBatch,
    widget::{BoxedWidget, Widget},
};

pub struct Button {
    pub event_rack: EventRack,
}

impl Button {
    pub fn new() -> Self {
        Self {
            event_rack: EventRack::new(),
        }
    }

    pub fn subscribe_event<E: EventMessage + 'static, B: SystemBatch>(
        mut self,
        systems: B,
    ) -> Self {
        self.event_rack.subscribe(TypeId::of::<E>(), systems);
        self
    }
}

impl Widget for Button {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        self.event_rack.fetch(event_type)
    }

    fn children_mut(&mut self) -> Vec<BoxedWidget> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
