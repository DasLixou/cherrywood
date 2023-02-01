use std::any::TypeId;

use hashbrown::HashMap;

use crate::{holding_ptr::HoldingPtr, resource::Resource};

pub struct Resources {
    data: HashMap<TypeId, HoldingPtr>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert_resource<R: Resource + 'static>(&mut self, value: R) {
        let ptr = HoldingPtr::new(value);
        self.data.insert(TypeId::of::<R>(), ptr);
    }

    pub fn get<R: Resource + 'static>(&self) -> Option<*mut u8> {
        self.data
            .get(&TypeId::of::<R>())
            .map(|ptr| ptr.as_ptr_mut())
    }
}
