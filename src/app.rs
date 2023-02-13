use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
};

use slotmap::{HopSlotMap, Key, SecondaryMap};

use crate::{
    batch::event::EventBatch,
    event::{Event, EventKind},
    resource::Resource,
    resources::Resources,
    system_context::SystemContext,
    utils::iter_choice::IterChoice,
    widget::{BoxedWidget, Widget},
    widget_context::WidgetContext,
    widgets::WidgetId,
};

pub struct App {
    resources: Resources,
    event_chan: Receiver<Event>,
    event_sender: Sender<Event>,
    main: WidgetId,
    widgets: HopSlotMap<WidgetId, BoxedWidget>,
    children: SecondaryMap<WidgetId, Vec<WidgetId>>,
    parents: SecondaryMap<WidgetId, WidgetId>,
}

impl App {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Event>();
        Self {
            resources: Resources::new(),
            event_chan: receiver,
            event_sender: sender,
            main: WidgetId::null(),
            widgets: HopSlotMap::with_key(),
            children: SecondaryMap::new(),
            parents: SecondaryMap::new(),
        }
    }

    pub fn with_content<'w, W: Widget + 'static>(
        &mut self,
        widget: impl FnOnce(WidgetContext) -> &'w mut W,
    ) -> &mut Self {
        self.main = widget(WidgetContext { app: self }).id();
        self
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
        for event in events.into_iter() {
            self.event_sender.send(event).unwrap();
        }
    }

    pub fn handle(&mut self) {
        while let Some(mut event) = self.event_chan.try_recv().ok() {
            if let EventKind::Root = &event.kind {
                event.kind = EventKind::FallingFrom(self.main)
            }
            let mut deque = VecDeque::new();
            let mut called_systems = Vec::new();
            match &event.kind {
                EventKind::FallingFrom(w) | EventKind::BubbleIn(w) => deque.push_back(w.clone()),
                _ => panic!("Unexpanded EventKind"),
            }
            'events: while let Some(widget) = deque.pop_front() {
                let mut systems = self
                    .get_widget(widget)
                    .unwrap()
                    .fetch_events(event.message.type_id());
                for sys in &mut systems {
                    sys.borrow_mut().initialize();
                    let (result, _) = sys.borrow_mut().run(SystemContext {
                        app: self,
                        event: event.clone(),
                        widget,
                    });
                    called_systems.push(sys.to_owned());
                    if result.event_catched {
                        break 'events;
                    }
                }
                match &event.kind {
                    EventKind::FallingFrom(_) => deque.extend(self.children_of(widget)),
                    EventKind::BubbleIn(_) => {}
                    _ => panic!("Unexpanded EventKind"),
                }
            }
            for sys in called_systems {
                sys.borrow_mut().apply(self);
            }
        }
    }

    pub fn create_widget<W: Widget + 'static>(&mut self, f: impl FnOnce(WidgetId) -> W) -> &mut W {
        let id = self.widgets.insert_with_key(|k| Box::new(f(k)));
        unsafe {
            self.widgets
                .get_unchecked_mut(id)
                .as_any_mut()
                .downcast_mut()
                .unwrap_unchecked()
        }
    }

    pub fn get_widget(&self, widget: WidgetId) -> Option<&BoxedWidget> {
        self.widgets.get(widget)
    }

    pub fn get_widget_mut(&mut self, widget: WidgetId) -> Option<&mut BoxedWidget> {
        self.widgets.get_mut(widget)
    }

    pub fn children_of(&self, widget: WidgetId) -> impl Iterator<Item = WidgetId> + '_ {
        if let Some(children) = self.children.get(widget) {
            IterChoice::First(children.iter().cloned())
        } else {
            IterChoice::Second(std::iter::empty::<WidgetId>())
        }
    }

    pub fn extend_children(
        &mut self,
        widget: WidgetId,
        capacity: usize,
        children: impl Iterator<Item = WidgetId>,
    ) {
        let vec = self.children.entry(widget).unwrap().or_default();
        vec.reserve(capacity);
        vec.extend(children)
    }

    pub fn parent_of(&self, widget: WidgetId) -> Option<WidgetId> {
        self.parents.get(widget).cloned()
    }
}
