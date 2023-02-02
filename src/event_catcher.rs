use std::any::TypeId;

use hashbrown::HashMap;

use crate::{app::App, system::BoxedDescribedSystem, system_batch::SystemBatch};

pub struct EventCatcher {
    systems: HashMap<TypeId, Vec<BoxedDescribedSystem>>,
}

impl EventCatcher {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    pub fn subscribe<B: SystemBatch>(&mut self, event_type: TypeId, systems: B) {
        self.systems.reserve(B::CAPACITY);
        for system in systems.into_iter() {
            system.lock().unwrap().initialize();
            self.systems.entry(event_type).or_default().push(system);
        }
    }

    pub fn run(&mut self, event_type: TypeId, app: &mut App) {
        for sys in self.systems.get_mut(&event_type).unwrap() {
            sys.lock().unwrap().run(app);
        }
    }

    pub fn fetch(&self, event_type: TypeId) -> Vec<BoxedDescribedSystem> {
        match self.systems.get(&event_type) {
            Some(vec) => vec.clone(),
            None => vec![],
        }
    }
}