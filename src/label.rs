use crate::{
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
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
