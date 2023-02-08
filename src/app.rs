use std::{
    cell::RefCell,
    collections::VecDeque,
    marker::PhantomData,
    rc::{Rc, Weak},
};

use crate::{
    batch::event::EventBatch,
    event::{Event, EventKind},
    resource::Resource,
    resources::Resources,
    system_context::SystemContext,
    widget::{BoxedWidget, Widget},
    widget_context::WidgetContext,
};

pub struct App {
    pub(crate) resources: Resources,
    widget: BoxedWidget,
    request_events: bool,
    event_queue: Vec<Event>,
}

impl App {
    pub fn new<W: Widget + 'static>(
        widget: impl FnOnce(WidgetContext<'_>) -> Rc<RefCell<W>>,
    ) -> Self {
        Self {
            resources: Resources::new(),
            widget: widget(WidgetContext {
                parent: Weak::<RefCell<W>>::new(),
                phantom: PhantomData,
            }),
            request_events: false,
            event_queue: Vec::new(),
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

    pub fn queue_events(&mut self, events: impl EventBatch) {
        self.event_queue.extend(events.into_iter());
        self.request_events = true;
    }

    pub fn handle(&mut self) {
        while self.request_events {
            self.request_events = false;
            while let Some(mut event) = self.event_queue.pop() {
                if let EventKind::Root = &event.kind {
                    event.kind = EventKind::FallingFrom(self.widget.clone())
                }
                let mut deque = VecDeque::new();
                let mut called_systems = Vec::new();
                match &event.kind {
                    EventKind::FallingFrom(w) | EventKind::BubbleIn(w) => {
                        deque.push_back(w.clone())
                    }
                    _ => panic!("Unexpanded EventKind"),
                }
                while let Some(widget) = deque.pop_front() {
                    let mut systems = widget.borrow_mut().fetch_events(event.message.type_id());
                    for sys in &mut systems {
                        sys.borrow_mut().initialize();
                        sys.borrow_mut().run(SystemContext {
                            app: self,
                            event: Some(event.clone()),
                            widget: &widget,
                        });
                        called_systems.push(sys.to_owned());
                    }
                    match &event.kind {
                        EventKind::FallingFrom(_) => {
                            deque.extend(widget.borrow_mut().children_mut().iter())
                        }
                        EventKind::BubbleIn(_) => {}
                        _ => panic!("Unexpanded EventKind"),
                    }
                }
                for sys in called_systems {
                    sys.borrow_mut().apply(self);
                }
            }
        }
    }
}
