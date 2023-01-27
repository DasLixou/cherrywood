use std::ops::Deref;

use crate::{access::Access, container::Container, resource::Resource};

pub trait SystemParam: Sized {
    type Param<'c>: SystemParam;

    fn initialize(access: &mut Access);

    fn get_param<'c>(container: &'c Container) -> Self::Param<'c>;
}

pub struct Res<'r, R: Resource> {
    data: &'r R,
}

impl<'r, R: Resource + 'static> SystemParam for Res<'r, R> {
    type Param<'c> = Res<'c, R>;

    fn initialize(access: &mut Access) {
        access.with_write::<R>();
    }

    fn get_param<'c>(container: &'c Container) -> Self::Param<'c> {
        Res {
            data: container.get_resource::<R>().expect(&format!(
                "Couldn't find resource for {}",
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
