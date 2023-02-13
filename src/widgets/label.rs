use std::{any::TypeId, cell::RefCell, rc::Rc};

use crate::{
    app::App,
    system::{BoxedDescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::Widget,
    widget_context::WidgetContext,
};

use super::WidgetId;

pub struct Label {
    content: Option<BoxedDescribedSystem<String>>,
    id: WidgetId,
    app: *mut App,
}

impl Label {
    pub fn new<'c>(cx: WidgetContext) -> &'c mut Self {
        unsafe {
            cx.app.as_mut().unwrap().create_widget(|id| Self {
                content: None,
                app: cx.app,
                id,
            })
        }
    }

    pub fn with_content<F: IntoDescribedSystem<String, Params>, Params: SystemParam>(
        &mut self,
        system: F,
    ) -> &mut Self {
        let system = Rc::new(RefCell::new(system.into_described()));
        self.content = Some(system);
        self
    }
}

impl Widget for Label {
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
