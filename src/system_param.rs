use std::ops::{Deref, DerefMut};

use crate::{access::Access, event::Event, resource::Resource, system_context::SystemContext};

pub trait SystemParam: Sized {
    type Param<'c>: SystemParam;

    fn initialize(access: &mut Access);

    fn get_param<'c>(context: &'c mut SystemContext<'_>) -> Self::Param<'c>;
}

pub struct Res<'r, R: Resource> {
    data: &'r R,
}

impl<'r, R: Resource + 'static> SystemParam for Res<'r, R> {
    type Param<'c> = Res<'c, R>;

    fn initialize(access: &mut Access) {
        access.with_read::<R>();
    }

    fn get_param<'c>(context: &'c mut SystemContext<'_>) -> Self::Param<'c> {
        Res {
            data: context.app.get_resource::<R>().expect(&format!(
                "Couldn't find resource {}",
                std::any::type_name::<R>()
            )),
        }
    }
}

impl<'r, R: Resource> Deref for Res<'r, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

pub struct ResMut<'r, R: Resource> {
    data: &'r mut R,
}

impl<'r, R: Resource + 'static> SystemParam for ResMut<'r, R> {
    type Param<'c> = ResMut<'c, R>;

    fn initialize(access: &mut Access) {
        access.with_write::<R>();
    }

    fn get_param<'c>(context: &'c mut SystemContext<'_>) -> Self::Param<'c> {
        ResMut {
            data: context.app.get_resource_mut::<R>().expect(&format!(
                "Couldn't find resource {}",
                std::any::type_name::<R>()
            )),
        }
    }
}

impl<'r, R: Resource> Deref for ResMut<'r, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'r, R: Resource> DerefMut for ResMut<'r, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

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
