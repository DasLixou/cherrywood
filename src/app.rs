use slotmap::HopSlotMap;

use crate::{
    resource::Resource,
    resources::Resources,
    widget::{Widget, WidgetId},
};

pub struct App {
    pub(crate) resources: Resources,
    widgets: HopSlotMap<WidgetId, Box<dyn Widget>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            widgets: HopSlotMap::with_key(),
        }
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

    pub fn new_widget<W: Widget + 'static>(&mut self, f: impl FnOnce(WidgetId) -> W) -> &mut W {
        let id = self.widgets.insert_with_key(|key| Box::new(f(key)));
        self.widgets
            .get_mut(id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<W>()
            .unwrap()
    }
}
