use std::{any::TypeId, cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    event::{Event, EventMessage},
    ptr::holding_ptr::HoldingPtr,
    resource::Resource,
    resources::Resources,
    system_context::SystemContext,
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

    pub fn dispatch_event<E: EventMessage + 'static>(&mut self, event: Event<E>) {
        // TODO: actually use `EventKind` in logic
        let mut deque = VecDeque::new();
        deque.push_back(self.widget.clone());
        let mut called_systems = Vec::new();
        while let Some(widget) = deque.pop_front() {
            let mut systems = widget.borrow_mut().fetch_events(TypeId::of::<E>());
            for sys in &mut systems {
                sys.borrow_mut().initialize();
                sys.borrow_mut().run(SystemContext {
                    app: self,
                    event: Some(HoldingPtr::new(event.clone())),
                });
                called_systems.push(sys.to_owned());
            }
            deque.extend(widget.borrow_mut().children_mut());
        }
        for sys in called_systems {
            sys.borrow_mut().apply(self); // TODO: when the system is borrowed, it can't call itself with dispatch_event - think about that
        }
    }
}
