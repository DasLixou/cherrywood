use crate::{resource::Resource, resources::Resources};

pub struct Container {
    pub(crate) resources: Resources,
}

impl Container {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
        }
    }

    pub fn insert_resource<R: Resource + 'static>(&mut self, value: R) {
        self.resources.insert_resource(value);
    }

    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources
            .get_resource::<R>()
            .map(|raw| unsafe { &*raw.cast::<R>() })
    }
}
