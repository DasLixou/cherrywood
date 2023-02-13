use std::any::TypeId;

use crate::{
    app::App, batch::SystemBatch, event::EventMessage, event_rack::EventRack,
    system::BoxedDescribedSystem, widget::Widget, widget_context::WidgetContext,
};

use super::WidgetId;

pub struct Button {
    event_rack: EventRack,
    id: WidgetId,
    app: *mut App,
}

impl Button {
    pub fn new<'c>(cx: WidgetContext) -> &'c mut Self {
        unsafe {
            cx.app.as_mut().unwrap().create_widget(|id| Self {
                event_rack: EventRack::new(),
                app: cx.app,
                id,
            })
        }
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
    fn fetch_events(&self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        self.event_rack.fetch(event_type)
    }

    fn id(&self) -> WidgetId {
        self.id
    }
}
