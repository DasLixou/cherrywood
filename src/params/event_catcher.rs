use std::marker::PhantomData;

use crate::{
    access::Access,
    app::App,
    event::{Event, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
    system_result::SystemResult,
};

pub struct EventCatcher<E: EventMessage> {
    data: Option<Event>,
    phantom: PhantomData<E>,
}

impl<E: EventMessage + Clone + 'static> EventCatcher<E> {
    pub fn get(&self) -> &E {
        self.data
            .as_ref()
            .expect("Event was already catched")
            .message
            .as_any()
            .downcast_ref()
            .expect("Expected other type of EventMessage than given!")
    }

    // TODO: catch function which should also cancel running the event further
    pub fn catch(&self) -> E {
        self.data
            .as_ref()
            .expect("Event was already catched")
            .message
            .as_any()
            .downcast_ref::<E>()
            .expect("Expected other type of EventMessage than given!")
            .clone()
    }
}

impl<E: EventMessage + 'static> SystemParam for EventCatcher<E> {
    type State = ();
    type Param<'c> = EventCatcher<E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(_state: &mut Self::State, context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventCatcher {
            data: Some(context.event.clone()),
            phantom: PhantomData,
        }
    }

    fn result(&mut self, result: &mut SystemResult) {
        if self.data.is_none() {
            result.event_catched = true;
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}
