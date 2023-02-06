use std::{any::TypeId, cell::RefCell, rc::Rc};

use crate::{
    children::Children,
    system::{BoxedDescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::{BoxedWidget, Widget},
    widget_handle::WidgetHandle,
};

pub struct Label {
    content: Option<BoxedDescribedSystem<String>>,
    parent: BoxedWidget,
}

impl Label {
    pub fn new(parent: BoxedWidget, children: &mut Children) -> WidgetHandle<Self> {
        children.add(Self {
            content: None,
            parent,
        })
    }
}

impl WidgetHandle<Label> {
    pub fn with_content<F: IntoDescribedSystem<String, Params>, Params: SystemParam>(
        &mut self,
        system: F,
    ) -> &mut Self {
        let system = Rc::new(RefCell::new(system.into_described()));
        self.data.borrow_mut().content = Some(system);
        self
    }
}

impl Widget for Label {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn parent(&mut self) -> BoxedWidget {
        self.parent.clone()
    }

    fn children_mut(&mut self) -> Children {
        Children::NONE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
