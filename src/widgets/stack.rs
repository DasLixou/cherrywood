use std::any::TypeId;

use crate::{
    app::App, batch::widget::WidgetBatch, system::BoxedDescribedSystem, widget::Widget,
    widget_context::WidgetContext,
};

use super::WidgetId;

pub struct Stack {
    id: WidgetId,
    app: *mut App,
}

impl Stack {
    pub fn new<'c>(cx: WidgetContext) -> &'c mut Self {
        unsafe {
            cx.app
                .as_mut()
                .unwrap()
                .create_widget(|id| Self { app: cx.app, id })
        }
    }

    pub fn extend_children<B: WidgetBatch>(
        &mut self,
        children: impl FnOnce(WidgetContext) -> B,
    ) -> &mut Self {
        let batch = children(WidgetContext { app: self.app });
        unsafe {
            self.app
                .as_mut()
                .unwrap()
                .extend_children(self.id, B::CAPACITY, batch.into_iter());
        }
        self
    }
}

impl Widget for Stack {
    fn fetch_events(&self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
