use std::any::TypeId;

use crate::{
    batch::WidgetBatch,
    children::Children,
    system::BoxedDescribedSystem,
    widget::{BoxedWidget, Widget},
    widget_handle::WidgetHandle,
};

pub struct Stack {
    children: Children,
    parent: BoxedWidget,
}

impl Stack {
    pub fn new(parent: BoxedWidget, children: &mut Children) -> WidgetHandle<Self> {
        children.add(Self {
            children: Children::new(),
            parent,
        })
    }
}

impl WidgetHandle<Stack> {
    pub fn with_children<B: WidgetBatch>(
        &mut self,
        children: impl FnOnce(BoxedWidget, &mut Children) -> B,
    ) -> &mut Self {
        children(self.data.clone(), &mut self.data.borrow_mut().children);
        self
    }
}

impl Widget for Stack {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn parent(&mut self) -> BoxedWidget {
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
