use crate::{
    container::Container,
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
};

pub struct Event {
    systems: Vec<BoxedDescribedSystem>,
}

impl Event {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn subscribe<F: IntoDescribedSystem<(), Params>, Params: SystemParam>(
        &mut self,
        system: F,
    ) {
        let mut system = Box::new(system.into_described());
        system.initialize();
        self.systems.push(system);
    }

    pub fn run(&mut self, container: &mut Container) {
        for sys in &mut self.systems {
            sys.run(container);
        }
    }
}
