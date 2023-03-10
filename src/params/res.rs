use std::ops::Deref;

use crate::{
    access::Access, app::App, resource::Resource, system_context::SystemContext,
    system_param::SystemParam, system_result::SystemResult,
};

pub struct Res<'r, R: Resource> {
    data: &'r R,
}

impl<R: Resource + 'static> SystemParam for Res<'_, R> {
    type State = ();
    type Param<'s> = Res<'s, R>;

    fn initialize(access: &mut Access) -> Self::State {
        access.with_read::<R>();
    }

    fn get_param<'c>(
        _state: &mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c> {
        Res {
            data: context.app.get_resource::<R>().expect(&format!(
                "Couldn't find resource {}",
                std::any::type_name::<R>()
            )),
        }
    }

    fn result(_state: &mut Self::State, _result: &mut SystemResult) {}

    fn apply<'a>(_state: Self::State, _app: &'a mut App) {}
}

impl<R: Resource> Deref for Res<'_, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}
