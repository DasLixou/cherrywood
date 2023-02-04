use std::any::TypeId;

use crate::{
    batch::WidgetBatch,
    system::BoxedDescribedSystem,
    widget::{BoxedWidget, Widget},
};

pub struct Stack {
    pub children: Vec<BoxedWidget>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_children<B: WidgetBatch>(mut self, children: B) -> Self {
        self.children.reserve(B::CAPACITY);
        self.children.extend(children.into_iter());
        self
    }
}

impl Widget for Stack {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<BoxedWidget> {
        self.children.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
