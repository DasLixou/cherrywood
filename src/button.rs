use crate::{
    system::{BoxedDescribedSystem, IntoDescribedSystem},
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
        self.on_click = Some(Box::new(callback.into_described()));
        self
    }
}
