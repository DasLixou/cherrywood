use std::ops::Deref;

use crate::{
    access::Access,
    app::App,
    event::{Event, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
};

pub struct EventCatcher<E: EventMessage> {
    data: Event<E>,
}

impl<E: EventMessage + 'static> SystemParam for EventCatcher<E> {
    type State = ();
    type Param<'c> = EventCatcher<E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(_state: &mut Self::State, context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventCatcher {
            data: (context
                .event
                .as_ref()
                .unwrap()
                .borrow_as::<Event<E>>()
                .clone()),
        }
    }

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}

impl<E: EventMessage> Deref for EventCatcher<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.data.message
    }
}
