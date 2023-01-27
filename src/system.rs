use std::marker::PhantomData;

use crate::{container::Container, system_param::SystemParam};

pub trait IntoDescribedSystem<Params> {
    type System: DescribedSystem + 'static;

    fn into_described(self) -> Self::System;
}

pub trait DescribedSystem {
    fn run(&mut self, container: &mut Container);
}

pub type BoxedDescribedSystem = Box<dyn DescribedSystem>;

pub struct FunctionSystem<F, Params> {
    system: F,
    phantom: PhantomData<Params>,
}

impl<F, Params: SystemParam + 'static> IntoDescribedSystem<Params> for F
where
    F: SystemParamFunction<Params> + 'static,
{
    type System = FunctionSystem<F, Params>;

    fn into_described(self) -> Self::System {
        FunctionSystem {
            system: self,
            phantom: PhantomData,
        }
    }
}

impl<F, Params: SystemParam> DescribedSystem for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Params> + 'static,
{
    fn run(&mut self, container: &mut Container) {
        SystemParamFunction::run(&mut self.system, container)
    }
}

pub(crate) trait SystemParamFunction<Params: SystemParam>: 'static {
    fn run(&mut self, container: &mut Container);
}
