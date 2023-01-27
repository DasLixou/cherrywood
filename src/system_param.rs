use crate::{container::Container, resource::Resource};

pub trait SystemParam<'c>: Sized {
    fn get_param(container: &'c Container) -> Self;
}

pub struct Res<'r, R: Resource> {
    data: &'r R,
}

impl<'c, R: Resource + 'static> SystemParam<'c> for Res<'c, R> {
    fn get_param(container: &'c Container) -> Self {
        Res {
            data: container.get_resource::<R>().expect(&format!(
                "Couldn't find resource for {}",
                std::any::type_name::<R>()
            )),
        }
    }
}
