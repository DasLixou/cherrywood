use crate::{access::Access, app::App, system_context::SystemContext, system_result::SystemResult};

pub trait SystemParam: Sized {
    type State;
    type Param<'c>: SystemParam;

    fn initialize(access: &mut Access) -> Self::State;

    fn get_param<'c>(
        state: &'c mut Self::State,
        context: &'c mut SystemContext<'_>,
    ) -> Self::Param<'c>;

    fn result(&mut self, result: &mut SystemResult);

    fn apply<'a>(state: Self::State, app: &'a mut App);
}
