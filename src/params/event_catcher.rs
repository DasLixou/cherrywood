use std::{marker::PhantomData, ops::Deref};

use crate::{
    access::Access,
    app::App,
    event::{Event, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
};

// TODO: add try() and catch() methods for this. catch will block from going further in other widgets
pub struct EventCatcher<E: EventMessage> {
    data: Event,
    phantom: PhantomData<E>,
}

impl<E: EventMessage + 'static> SystemParam for EventCatcher<E> {
    type State = ();
    type Param<'c> = EventCatcher<E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(_state: &mut Self::State, context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventCatcher {
            data: context.event.as_ref().unwrap().clone(),
            phantom: PhantomData,
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}

impl<E: EventMessage> Deref for EventCatcher<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.data
            .message
            .as_any()
            .downcast_ref()
            .expect("Expected other type of EventMessage than given!")
    }
}
