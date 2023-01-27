use std::any::TypeId;

use bevy_ptr::OwningPtr;
use hashbrown::HashMap;

use crate::resource::Resource;

pub struct Resources {
    data: HashMap<TypeId, *mut u8>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert_resource<R: Resource + 'static>(&mut self, value: R) {
        OwningPtr::make(value, |ptr| {
            self.data.insert(TypeId::of::<R>(), ptr.as_ptr())
        });
    }

    pub fn get<R: Resource + 'static>(&self) -> Option<&*mut u8> {
        self.data.get(&TypeId::of::<R>())
    }
}
