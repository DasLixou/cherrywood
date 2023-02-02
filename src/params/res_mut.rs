use std::ops::{Deref, DerefMut};

use crate::{
    access::Access, resource::Resource, system_context::SystemContext, system_param::SystemParam,
};

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
