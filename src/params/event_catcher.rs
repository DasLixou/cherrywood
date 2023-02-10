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
    data: Event,
    catched: &'s mut bool,
    phantom: PhantomData<E>,
}

impl<E: EventMessage + Clone + 'static> EventCatcher<'_, E> {
    pub fn get(&self) -> &E {
        self.data
            .message
            .as_any()
            .downcast_ref()
            .expect("Expected other type of EventMessage than given!")
    }

    // TODO: catch function which should also cancel running the event further
    pub fn catch(&mut self) -> E {
        *self.catched = true;
        self.data
            .message
            .as_any()
            .downcast_ref::<E>()
            .expect("Expected other type of EventMessage than given!")
            .clone()
    }
}

impl<E: EventMessage + 'static> SystemParam for EventCatcher<'_, E> {
    type State = bool;
    type Param<'c> = EventCatcher<'c, E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
        false
    }

    fn get_param<'c>(
        state: &'c mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        EventCatcher {
            data: context.event.clone(),
            catched: state,
            phantom: PhantomData,
        }
    }

    fn result(state: &mut Self::State, result: &mut SystemResult) {
        if *state {
            result.event_catched = true;
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}
