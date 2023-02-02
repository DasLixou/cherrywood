use std::{any::TypeId, cell::RefCell, rc::Rc};

use crate::{
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::{BoxedWidget, Widget},
};

pub struct Label {
    pub content: Option<BoxedDescribedSystem<String>>,
}

impl Label {
    pub fn new() -> Self {
        Self { content: None }
    }

    pub fn with_content<F: IntoDescribedSystem<String, Params>, Params: SystemParam>(
        mut self,
        system: F,
    ) -> Self {
        let system = Rc::new(RefCell::new(system.into_described()));
        system.borrow_mut().initialize();
        self.content = Some(system);
        self
    }
}

impl Widget for Label {
    fn fetch_events(&mut self, _event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<BoxedWidget> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
