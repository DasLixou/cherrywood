use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::widget::{BoxedWidget, Widget};

#[derive(Clone)]
pub struct Children {
    children: Vec<BoxedWidget>,
}

impl Children {
    pub const NONE: Self = Children::new();

    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add<W: Widget + 'static>(
        &mut self,
        child: impl FnOnce(Weak<RefCell<W>>) -> W,
    ) -> Rc<RefCell<W>> {
        let rc = Rc::new_cyclic(|ptr| {
            let widget = child(ptr.clone());
            RefCell::new(widget)
        });
        self.children.push(rc.clone());
        rc
    }

    pub fn iter(&mut self) -> impl Iterator<Item = BoxedWidget> + '_ {
        self.children.iter().map(|rc| rc.clone())
    }
}
