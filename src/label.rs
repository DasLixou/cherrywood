use crate::{
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
        _t: std::any::TypeId,
        _ptr: crate::holding_ptr::HoldingPtr,
    ) -> Option<crate::holding_ptr::HoldingPtr> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
