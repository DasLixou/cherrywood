use std::{marker::PhantomData, ops::AddAssign};

use crate::{
    access::Access,
    app::App,
    event::{Event, EventKind, EventMessage},
    system_context::SystemContext,
    system_param::SystemParam,
    system_result::SystemResult,
    widget::BoxedWidget,
};

pub struct EventThrower<'w, E: EventMessage> {
    events: &'w mut Vec<Event>,
    phantom: PhantomData<E>,
    widget: &'w BoxedWidget,
}

impl<E: EventMessage + 'static> SystemParam for EventThrower<'_, E> {
    type State = Vec<Event>;
    type Param<'s> = EventThrower<'s, E>;

    fn initialize(_access: &mut Access) -> Self::State {
        //access.with_read::<R>(); // TODO: access with multiple types
        vec![]
    }

    fn get_param<'c>(
        state: &'c mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        EventThrower {
            events: state,
            phantom: PhantomData,
            widget: context.widget,
        }
    }

    fn result(_state: &mut Self::State, _result: &mut SystemResult) {}

    fn apply<'a>(state: Self::State, app: &'a mut App) {
        app.queue_events(state);
    }
}

impl<'w, E: EventMessage + 'static> AddAssign<(E, EventKind)> for EventThrower<'w, E> {
    fn add_assign(&mut self, rhs: (E, EventKind)) {
        let kind = match rhs.1 {
            EventKind::Root => EventKind::Root,
            EventKind::Falling => EventKind::FallingFrom(self.widget.clone()),
            EventKind::Climbing => todo!(),
            EventKind::Bubble => EventKind::BubbleIn(self.widget.clone()),
            k => k,
        };
        self.events.push(Event::new(rhs.0, kind));
    }
}
