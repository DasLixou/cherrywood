use std::marker::PhantomData;

use crate::{access::Access, container::Container, system_param::SystemParam};

pub trait IntoDescribedSystem<Params> {
    type System: DescribedSystem + 'static;

    fn into_described(self) -> Self::System;
}

pub trait DescribedSystem {
    fn initialize(&mut self);

    fn run(&mut self, container: &mut Container);
}

pub type BoxedDescribedSystem = Box<dyn DescribedSystem>;

pub struct FunctionSystem<F, Params> {
    system: F,
    access: Access,
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
            access: Access::new(),
            phantom: PhantomData,
        }
    }
}

impl<F, Params: SystemParam> DescribedSystem for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Params> + 'static,
{
    fn initialize(&mut self) {
        F::initialize(&mut self.access);
    }

    fn run(&mut self, container: &mut Container) {
        SystemParamFunction::run(&mut self.system, container);
    }
}

pub(crate) trait SystemParamFunction<Params: SystemParam>: 'static {
    fn initialize(access: &mut Access)
    where
        Self: Sized;

    fn run(&mut self, container: &mut Container);
}
