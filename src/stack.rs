use std::any::TypeId;

use crate::{app::App, holding_ptr::HoldingPtr, widget::Widget, widget_batch::WidgetBatch};

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
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
    fn dispatch_event(
        &mut self,
        app: &mut App,
        t: TypeId,
        mut ptr: HoldingPtr,
    ) -> Option<HoldingPtr> {
        for child in &mut self.children {
            if let Some(catch) = child.dispatch_event(app, t, ptr) {
                ptr = catch;
            } else {
                return None;
            }
        }
        Some(ptr)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
