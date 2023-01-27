use std::any::TypeId;

use hashbrown::{hash_map::Entry, HashMap};

pub enum AccessKind {
    Read,
    Write,
}

pub struct Access(HashMap<TypeId, AccessKind>);

impl Access {
    pub fn new() -> Self {
        Access(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Access(HashMap::with_capacity(capacity))
    }

    pub fn with_read<T: 'static>(&mut self) -> &mut Self {
        match self.0.entry(TypeId::of::<T>()) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                AccessKind::Read => {},
                AccessKind::Write => panic!("Can't require read access for {} because it was already requested with write access", std::any::type_name::<T>())
            },
            Entry::Vacant(entry) => { entry.insert(AccessKind::Read); },
        };
        self
    }

    pub fn with_write<T: 'static>(&mut self) -> &mut Self {
        match self.0.entry(TypeId::of::<T>()) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                AccessKind::Read => panic!("Can't require write access for {} because it was already requested with read access", std::any::type_name::<T>()),
                AccessKind::Write => panic!("Can't require write access for {} because it was already requested with write access", std::any::type_name::<T>())
            },
            Entry::Vacant(entry) => { entry.insert(AccessKind::Write); },
        };
        self
    }
}
