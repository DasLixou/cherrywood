use std::any::TypeId;

use crate::{
    event::Event,
    event_catcher::EventCatcher,
    system::BoxedDescribedSystem,
    system_batch::SystemBatch,
    widget::{BoxedWidget, Widget},
};

pub struct Button {
    pub event_catcher: EventCatcher,
}

impl Button {
    pub fn new() -> Self {
        Self {
            event_catcher: EventCatcher::new(),
        }
    }

    pub fn subscribe_event<E: Event + 'static, B: SystemBatch>(mut self, systems: B) -> Self {
        self.event_catcher.subscribe(TypeId::of::<E>(), systems);
        self
    }
}

impl Widget for Button {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        self.event_catcher.fetch(event_type)
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
