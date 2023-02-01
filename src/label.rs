use std::any::TypeId;

use crate::{
    app::App,
    holding_ptr::HoldingPtr,
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::Widget,
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
        let mut system = Box::new(system.into_described());
        system.initialize();
        self.content = Some(system);
        self
    }
}

impl Widget for Label {
    fn dispatch_event(
        &mut self,
        _app: &mut App,
        _t: TypeId,
        _ptr: HoldingPtr,
    ) -> Option<HoldingPtr> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
