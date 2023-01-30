use crate::{resource::Resource, resources::Resources, widget::Widget};

pub struct App {
    pub(crate) resources: Resources,
}

impl App {
    pub fn new<W: Widget>(main: impl FnOnce() -> W) -> Self {
        let app = Self {
            resources: Resources::new(),
        };
        let _widget = Box::new(main());
        app
    }

    pub fn insert_resource<R: Resource + 'static>(&mut self, value: R) {
        self.resources.insert_resource(value);
    }

    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources
            .get::<R>()
            .map(|raw| unsafe { &*raw.cast::<R>() })
    }

    pub fn get_resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources
            .get::<R>()
            .map(|raw| unsafe { &mut *raw.cast::<R>() })
    }
}
