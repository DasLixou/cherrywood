use std::ops::{Deref, DerefMut};

use crate::{
    access::Access, app::App, resource::Resource, system_context::SystemContext,
    system_param::SystemParam, system_result::SystemResult,
};

pub struct ResMut<'r, R: Resource> {
    data: &'r mut R,
}

impl<R: Resource + 'static> SystemParam for ResMut<'_, R> {
    type State = ();
    type Param<'s> = ResMut<'s, R>;

    fn initialize(access: &mut Access) -> Self::State {
        access.with_write::<R>();
    }

    fn get_param<'c>(
        _state: &mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        ResMut {
            data: context.app.get_resource_mut::<R>().expect(&format!(
                "Couldn't find resource {}",
                std::any::type_name::<R>()
            )),
        }
    }

    fn result(_state: &mut Self::State, _result: &mut SystemResult) {}

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}

impl<R: Resource> Deref for ResMut<'_, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<R: Resource> DerefMut for ResMut<'_, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
