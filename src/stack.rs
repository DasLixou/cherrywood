use crate::widget::Widget;

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    //pub fn with_children<const N: usize>(f: impl FnOnce(&Widget) -> [])
}

impl Widget for Stack {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
