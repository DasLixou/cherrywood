use std::ops::AddAssign;

use crate::{
    access::Access,
    app::App,
    event::{Event, EventKind, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
};

pub struct EventThrower<'w, E: EventMessage> {
    events: &'w mut Vec<Event<E>>,
}

impl<'w, E: EventMessage + 'static> SystemParam for EventThrower<'w, E> {
    type State = Vec<Event<E>>;
    type Param<'c> = EventThrower<'c, E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
        vec![]
    }

    fn get_param<'c>(
        state: &'c mut Self::State,
        _context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        EventThrower { events: state }
    }

    fn apply<'a>(state: Self::State, app: &'a mut App) {
        for event in state {
            app.dispatch_event(event);
        }
    }
}

impl<'w, E: EventMessage> AddAssign<(E, EventKind)> for EventThrower<'w, E> {
    fn add_assign(&mut self, rhs: (E, EventKind)) {
        self.events.push(Event::new(rhs.0, rhs.1));
    }
}
