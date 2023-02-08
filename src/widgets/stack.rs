use std::{
    any::TypeId,
    cell::RefCell,
    marker::PhantomData,
    rc::{Rc, Weak},
};

use crate::{
    batch::widget::WidgetBatch, children::Children, system::BoxedDescribedSystem, widget::Widget,
    widget_context::WidgetContext,
};

pub struct Stack {
    children: Children,
    parent: Weak<RefCell<dyn Widget>>,
    me: Weak<RefCell<Self>>,
}

impl Stack {
    pub fn new(cx: WidgetContext<'_>) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            let widget = Self {
                children: Children::new(),
                parent: cx.parent,
                me: me.clone(),
            };
            RefCell::new(widget)
        })
    }

    pub fn with_children<B: WidgetBatch>(
        &mut self,
        children: impl FnOnce(WidgetContext<'_>) -> B,
    ) -> &mut Self {
        let batch = children(WidgetContext {
            parent: self.parent.clone(),
            phantom: PhantomData,
        });
        self.children.extend_batch(batch);
        self
    }
}

impl Widget for Stack {
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
        self.children.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
