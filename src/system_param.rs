use std::ops::Deref;

use crate::{access::Access, container::Container, resource::Resource};

pub trait SystemParam: Sized {
    type Param<'c>: SystemParam;

    fn initialize(access: &mut Access);

    fn get_param<'c>(container: &'c mut Container) -> Self::Param<'c>;
}

pub struct Res<'r, R: Resource> {
    data: &'r R,
}

impl<'r, R: Resource + 'static> SystemParam for Res<'r, R> {
    type Param<'c> = Res<'c, R>;

    fn initialize(access: &mut Access) {
        access.with_read::<R>();
    }

    fn get_param<'c>(container: &'c mut Container) -> Self::Param<'c> {
        Res {
            data: container.get_resource::<R>().expect(&format!(
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

    fn get_param<'c>(container: &'c mut Container) -> Self::Param<'c> {
        ResMut {
            data: container.get_resource_mut::<R>().expect(&format!(
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
