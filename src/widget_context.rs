use std::{cell::RefCell, rc::Weak};

use crate::{children::Children, widget::Widget};

pub struct WidgetContext<'c> {
    pub parent: Weak<RefCell<dyn Widget>>,
    pub children: &'c mut Children,
}

impl<'c> Clone for WidgetContext<'c> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            children: unsafe {
                &mut *(self.children as *const Children as *mut Children) as &mut Children
            },
        }
    }
}
