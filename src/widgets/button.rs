use std::{
    any::TypeId,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    batch::SystemBatch, children::Children, event::EventMessage, event_rack::EventRack,
    system::BoxedDescribedSystem, widget::Widget, widget_context::WidgetContext,
};

pub struct Button {
    pub event_rack: EventRack,
    parent: Weak<RefCell<dyn Widget>>,
    me: Weak<RefCell<Self>>,
}

impl Button {
    pub fn new<'a>(cx: WidgetContext<'a>) -> Rc<RefCell<Self>> {
        cx.children.add(|me| Self {
            event_rack: EventRack::new(),
            parent: cx.parent,
            me,
        })
    }

    pub fn subscribe_event<E: EventMessage + 'static, B: SystemBatch>(
        &mut self,
        systems: B,
    ) -> &mut Self {
        self.event_rack.subscribe(TypeId::of::<E>(), systems);
        self
    }
}

impl Widget for Button {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        self.event_rack.fetch(event_type)
    }

    fn finish(&self) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        self.me.upgrade().unwrap()
    }

    fn parent(&mut self) -> Weak<RefCell<dyn Widget>> {
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
