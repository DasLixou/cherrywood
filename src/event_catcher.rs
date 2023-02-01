use crate::{app::App, system::BoxedDescribedSystem, system_batch::SystemBatch};

pub struct EventCatcher {
    systems: Vec<BoxedDescribedSystem>,
}

impl EventCatcher {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn subscribe<B: SystemBatch>(&mut self, systems: B) {
        self.systems.reserve(B::CAPACITY);
        for mut system in systems.into_iter() {
            system.initialize();
            self.systems.push(system);
        }
    }

    pub fn run(&mut self, app: &mut App) {
        for sys in &mut self.systems {
            sys.run(app);
        }
    }
}
