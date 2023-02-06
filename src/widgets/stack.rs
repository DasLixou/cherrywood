use std::{
    any::TypeId,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    batch::WidgetBatch, children::Children, system::BoxedDescribedSystem, widget::Widget,
    widget_context::WidgetContext,
};

pub struct Stack {
    children: Children,
    parent: Weak<RefCell<dyn Widget>>,
    me: Weak<RefCell<dyn Widget>>,
}

impl Stack {
    pub fn new(cx: WidgetContext<'_>) -> Rc<RefCell<Self>> {
        cx.children.add(|me| Self {
            children: Children::new(),
            parent: cx.parent,
            me,
        })
    }

    pub fn with_children<B: WidgetBatch>(
        &mut self,
        children: impl FnOnce(WidgetContext<'_>) -> B,
    ) -> &mut Self {
        children(WidgetContext {
            parent: self.parent.clone(),
            children: &mut self.children,
        });
        self
    }
}

impl Widget for Stack {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn parent(&mut self) -> Weak<RefCell<dyn Widget>> {
        self.parent.clone()
    }

    fn children_mut(&mut self) -> Children {
        self.children.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
