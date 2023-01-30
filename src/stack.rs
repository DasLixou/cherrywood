use crate::{widget::Widget, widget_batch::WidgetBatch};

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: impl WidgetBatch) -> Self {
        self.children.extend(children.into_iter());
        self
    }
}

impl Widget for Stack {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
