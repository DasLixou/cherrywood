use std::ops::Deref;

use crate::{
    access::Access, event::Event, system_context::SystemContext, system_param::SystemParam,
};

pub struct EventCatcher<E: Event> {
    data: E,
}

impl<E: Event + 'static> SystemParam for EventCatcher<E> {
    type Param<'c> = EventCatcher<E>;

    fn initialize(_access: &mut Access) {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventCatcher {
            data: (context.event.as_ref().unwrap().borrow_as::<E>().clone()),
        }
    }
}

impl<E: Event> Deref for EventCatcher<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
