use std::ops::Deref;

use crate::{
    access::Access, resource::Resource, system_context::SystemContext, system_param::SystemParam,
};

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
