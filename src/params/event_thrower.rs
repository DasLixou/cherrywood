use std::ops::AddAssign;

use crate::{
    access::Access, event::Event, system_context::SystemContext, system_param::SystemParam,
};

pub struct EventThrower<E: Event> {
    events: Vec<E>,
}

impl<E: Event + 'static> SystemParam for EventThrower<E> {
    type Param<'c> = EventThrower<E>;

    fn initialize(_access: &mut Access) {
        //access.with_read::<R>(); // TODO: access with multiple types
    }

    fn get_param<'c>(_context: &mut SystemContext<'c>) -> Self::Param<'c> {
        EventThrower { events: vec![] }
    }
}

impl<E: Event> AddAssign<E> for EventThrower<E> {
    fn add_assign(&mut self, rhs: E) {
        self.events.push(rhs);
    }
}
