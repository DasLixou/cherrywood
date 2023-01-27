use crate::{button::Button, resource::Resource, resources::Resources};

pub struct Container {
    main: Button,
    pub(crate) resources: Resources,
}

impl Container {
    pub fn new(main: Button) -> Self {
        Self {
            main,
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

    pub fn click(&mut self) {
        if let Some(on_click) = &mut self.main.on_click {
            on_click.run();
        }
    }
}
