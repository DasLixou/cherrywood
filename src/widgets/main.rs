use std::{any::TypeId, cell::RefCell, rc::Rc};

use crate::{
    children::Children,
    system::BoxedDescribedSystem,
    widget::{BoxedWidget, Widget},
    widget_handle::WidgetHandle,
};

pub(crate) struct Main {
    children: Children,
}

impl Main {
    pub(crate) fn new() -> WidgetHandle<Self> {
        WidgetHandle::new(Self {
            children: Children::new(),
        })
    }
}

impl WidgetHandle<Main> {
    pub fn with_child<W: Widget + 'static>(
        &mut self,
        child: impl FnOnce(BoxedWidget, &mut Children) -> Rc<RefCell<W>>,
    ) -> &mut Self {
        let child = child(self.data.clone(), &mut self.data.borrow_mut().children);
        self.data.borrow_mut().children.add_directly(child);
        self
    }
}

impl Widget for Main {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn parent(&mut self) -> BoxedWidget {
        unimplemented!()
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
