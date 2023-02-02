use crate::{access::Access, system_context::SystemContext};

pub trait SystemParam: Sized {
    type Param<'c>: SystemParam;

    fn initialize(access: &mut Access);

    fn get_param<'c>(context: &'c mut SystemContext<'_>) -> Self::Param<'c>;
}
