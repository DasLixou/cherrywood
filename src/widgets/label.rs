use std::{
    any::TypeId,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    children::Children,
    system::{BoxedDescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::Widget,
    widget_context::WidgetContext,
};

pub struct Label {
    content: Option<BoxedDescribedSystem<String>>,
    parent: Weak<RefCell<dyn Widget>>,
    me: Weak<RefCell<Self>>,
}

impl Label {
    pub fn new(cx: WidgetContext<'_>) -> Rc<RefCell<Self>> {
        cx.children.add(|me| Self {
            content: None,
            parent: cx.parent,
            me,
        })
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
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
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
