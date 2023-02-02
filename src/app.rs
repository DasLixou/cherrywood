use std::{any::TypeId, cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    event::Event,
    resource::Resource,
    resources::Resources,
    widget::{BoxedWidget, Widget},
};

pub struct App {
    pub(crate) resources: Resources,
    widget: BoxedWidget,
}

impl App {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            resources: Resources::new(),
            widget: Rc::new(RefCell::new(widget)),
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

    pub fn dispatch_event<E: Event + 'static>(&mut self, _event: E) {
        // TODO: use event
        let mut deque = VecDeque::new();
        deque.push_back(self.widget.clone());
        while let Some(widget) = deque.pop_front() {
            let mut systems = widget.borrow_mut().fetch_events(TypeId::of::<E>());
            for sys in &mut systems {
                sys.borrow_mut().run(self);
            }
            deque.extend(widget.borrow_mut().children_mut());
        }
    }
}
