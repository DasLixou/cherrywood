use std::any::TypeId;

use crate::{
    event::Event, holding_ptr::HoldingPtr, resource::Resource, resources::Resources, widget::Widget,
};

pub struct App {
    pub(crate) resources: Resources,
    widget: Box<dyn Widget>,
}

impl App {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            resources: Resources::new(),
            widget: Box::new(widget),
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

    pub fn dispatch_event<E: Event + 'static>(&mut self, event: E) -> Option<E> {
        // TODO: can we make this safe?
        unsafe {
            let ptr: *mut Self = self;
            (&mut *ptr)
                .widget
                .dispatch_event(self, TypeId::of::<E>(), HoldingPtr::new(event))
                .map(|ret| ret.destroy_as::<E>())
        }
    }
}
