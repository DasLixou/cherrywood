use std::marker::PhantomData;

use crate::system_param::SystemParam;

pub trait IntoDescribedSystem<Params> {
    type System: DescribedSystem + 'static;

    fn into_described(self) -> Self::System;
}

pub trait DescribedSystem {
    fn run(&mut self);
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
    fn run(&mut self) {
        SystemParamFunction::run(&mut self.system)
    }
}

pub(crate) trait SystemParamFunction<Params: SystemParam>: 'static {
    fn run(&mut self);
}
