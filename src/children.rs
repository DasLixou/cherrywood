use std::{cell::RefCell, rc::Rc};

use crate::{
    widget::{BoxedWidget, Widget},
    widget_handle::WidgetHandle,
};

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

    pub fn add<W: Widget + 'static>(&mut self, child: W) -> WidgetHandle<W> {
        let mut handle = WidgetHandle::new(child);
        self.children.push(handle.finish());
        handle
    }

    pub(crate) fn add_directly<W: Widget + 'static>(&mut self, child: Rc<RefCell<W>>) {
        self.children.push(child);
    }

    pub fn iter(&mut self) -> impl Iterator<Item = BoxedWidget> + '_ {
        self.children.iter().map(|rc| rc.clone())
    }
}
