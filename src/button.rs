use crate::{
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
};

pub struct Button {
    pub on_click: Option<BoxedDescribedSystem>,
}

impl Button {
    pub fn new() -> Self {
        Self { on_click: None }
    }

    pub fn on_click<F: IntoDescribedSystem<Params>, Params: SystemParam>(
        mut self,
        callback: F,
    ) -> Self {
        let mut system = Box::new(callback.into_described());
        system.initialize();
        self.on_click = Some(system);
        self
    }
}
