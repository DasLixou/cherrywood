use std::{cell::RefCell, rc::Rc};

use crate::{
    access::Access, app::App, system_context::SystemContext, system_param::SystemParam,
    system_result::SystemResult,
};

pub trait IntoDescribedSystem<Out, Params> {
    type System: DescribedSystem<Out> + 'static;

    fn into_described(self) -> Self::System;
}

pub trait DescribedSystem<Out> {
    fn initialize(&mut self);

    fn run<'c>(&mut self, context: SystemContext<'c>) -> (SystemResult, Out);

    fn apply<'a>(&mut self, app: &'a mut App);
}

pub type BoxedDescribedSystem<Out = ()> = Rc<RefCell<dyn DescribedSystem<Out>>>;

pub struct FunctionSystem<F, Params: SystemParam> {
    system: F,
    access: Access,
    state: Option<Params::State>,
}

impl<F, Out, Params: SystemParam + 'static> IntoDescribedSystem<Out, Params> for F
where
    F: SystemParamFunction<Out, Params> + 'static,
{
    type System = FunctionSystem<F, Params>;

    fn into_described(self) -> Self::System {
        FunctionSystem {
            system: self,
            access: Access::new(),
            state: None,
        }
    }
}

impl<F, Out, Params: SystemParam> DescribedSystem<Out> for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Out, Params> + 'static,
{
    fn initialize(&mut self) {
        self.access.clear();
        self.state = Some(F::initialize(&mut self.access));
    }

    fn run<'c>(&mut self, context: SystemContext<'c>) -> (SystemResult, Out) {
        SystemParamFunction::run(
            &mut self.system,
            self.state
                .as_mut()
                .expect("State from System was uninitialized!"),
            context,
        )
    }

    fn apply<'a>(&mut self, app: &'a mut App) {
        let state = self
            .state
            .take()
            .expect("State from System was uninitialized! Maybe the buffers were already applied?");
        SystemParamFunction::apply(&mut self.system, state, app)
    }
}

pub(crate) trait SystemParamFunction<Out, Params: SystemParam>: 'static {
    fn initialize(access: &mut Access) -> Params::State
    where
        Self: Sized;

    fn run<'c>(
        &mut self,
        state: &mut Params::State,
        context: SystemContext<'c>,
    ) -> (SystemResult, Out);

    fn apply<'a>(&mut self, state: Params::State, app: &'a mut App);
}
