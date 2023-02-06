use std::any::TypeId;

use crate::{
    batch::SystemBatch,
    children::Children,
    event::EventMessage,
    event_rack::EventRack,
    system::BoxedDescribedSystem,
    widget::{BoxedWidget, Widget},
    widget_handle::WidgetHandle,
};

pub struct Button {
    pub event_rack: EventRack,
    parent: BoxedWidget,
}

impl Button {
    pub fn new(parent: BoxedWidget, children: &mut Children) -> WidgetHandle<Self> {
        children.add(Self {
            event_rack: EventRack::new(),
            parent,
        })
    }
}

impl WidgetHandle<Button> {
    pub fn subscribe_event<E: EventMessage + 'static, B: SystemBatch>(
        &mut self,
        systems: B,
    ) -> &mut Self {
        self.data
            .borrow_mut()
            .event_rack
            .subscribe(TypeId::of::<E>(), systems);
        self
    }
}

impl Widget for Button {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        self.event_rack.fetch(event_type)
    }

    fn parent(&mut self) -> BoxedWidget {
        self.parent.clone()
    }

    fn children_mut(&mut self) -> Children {
        Children::NONE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
