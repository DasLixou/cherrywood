use std::marker::PhantomData;

use crate::{
    access::Access,
    app::App,
    event::{Event, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
    system_result::SystemResult,
};

pub struct EventCatcher<'s, E: EventMessage> {
    state: &'s mut Option<Event>,
    phantom: PhantomData<E>,
}

impl<E: EventMessage + Clone + 'static> EventCatcher<'_, E> {
    pub fn get(&self) -> &E {
        self.state
            .as_ref()
            .expect("Event was already catched")
            .message
            .as_any()
            .downcast_ref()
            .expect("Expected other type of EventMessage than given!")
    }

    pub fn catch(&mut self) -> E {
        self.state
            .take()
            .expect("Event was already catched")
            .message
            .as_any()
            .downcast_ref::<E>()
            .expect("Expected other type of EventMessage than given!")
            .clone()
    }
}

impl<E: EventMessage + 'static> SystemParam for EventCatcher<'_, E> {
    type State = Option<Event>;
    type Param<'s> = EventCatcher<'s, E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
        None
    }

    fn get_param<'c>(
        state: &'c mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        *state = Some(context.event.clone());
        EventCatcher {
            state,
            phantom: PhantomData,
        }
    }

    fn result(state: &mut Self::State, result: &mut SystemResult) {
        if state.is_none() {
            result.event_catched = true;
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}
