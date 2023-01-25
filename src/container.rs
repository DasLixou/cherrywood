use crate::{button::Button, resource::Resource, resources::Resources};

pub struct Container {
    main: Button,
    resources: Resources,
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

    pub fn click(&mut self) {
        self.main.on_click.run();
    }
}
