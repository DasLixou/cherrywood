use std::ops::AddAssign;

use crate::{
    access::Access,
    event::{Event, EventKind, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
};

pub struct EventThrower<E: EventMessage> {
    events: Vec<Event<E>>,
}

impl<E: EventMessage + 'static> SystemParam for EventThrower<E> {
    type Param<'c> = EventThrower<E>;

    fn initialize(_access: &mut Access) {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(_context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventThrower { events: vec![] }
    }
}

impl<E: EventMessage> AddAssign<(E, EventKind)> for EventThrower<E> {
    fn add_assign(&mut self, rhs: (E, EventKind)) {
        self.events.push(Event::new(rhs.0, rhs.1));
    }
}
