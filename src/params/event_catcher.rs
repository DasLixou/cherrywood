use std::marker::PhantomData;

use crate::{
    access::Access,
    app::App,
    event::{Event, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
};

pub struct EventCatcher<E: EventMessage> {
    data: Event,
    phantom: PhantomData<E>,
}

impl<E: EventMessage + Clone + 'static> EventCatcher<E> {
    pub fn get(&self) -> &E {
        self.data
            .message
            .as_any()
            .downcast_ref()
            .expect("Expected other type of EventMessage than given!")
    }

    // TODO: catch function which should also cancel running the event further
    pub fn catch(&self) -> E {
        self.data
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
            data: context.event.clone(),
            phantom: PhantomData,
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}
