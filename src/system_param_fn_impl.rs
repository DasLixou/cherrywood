use crate::access::Access;
use crate::system::SystemParamFunction;
use crate::system_context::SystemContext;
use crate::system_param::SystemParam;

impl SystemParam for () {
    type Param<'c> = ();

    fn initialize(_access: &mut Access) {}

    fn get_param<'c>(_context: &mut SystemContext<'c>) -> Self::Param<'c> {
        ()
    }
}

impl<F> SystemParamFunction<(), ()> for F
where
    F: Fn() -> () + 'static,
{
    fn initialize(_access: &mut Access)
    where
        Self: Sized,
    {
    }

    fn run<'c>(&mut self, _context: SystemContext<'c>) -> () {
        self();
        ()
    }
}

macro_rules! impl_system_param_fn {
    ($(($generic:ident, $index:tt))+) => {
        impl<$($generic: SystemParam),*> SystemParam for ($($generic),*,) {
            type Param<'c> = ($($generic::Param<'c>),*,);

            fn initialize(access: &mut Access) {
                $(
                    <$generic as SystemParam>::initialize(access);
                )*
            }

            fn get_param<'c>(context: &'c mut SystemContext<'_>) -> Self::Param<'c> {
                ($(
                    <$generic as SystemParam>::get_param(unsafe {
                        // SAFETY: we already checked for conflicts in `initialize`
                        &mut *((&mut *context) as *mut SystemContext)
                    })
                ),*,)
            }
        }

        impl<Func, Out, $($generic: SystemParam),*> SystemParamFunction<Out, ($($generic),*,)> for Func
        where
            Func: Fn($($generic),*) -> Out + Fn($($generic::Param<'_>),*) -> Out + 'static,
        {
            fn initialize(access: &mut Access)
            where
                Self: Sized
            {
                <($($generic),*,) as SystemParam>::initialize(access);
            }

            fn run<'c>(&mut self, mut context: SystemContext<'c>) -> Out {
                let params = <($($generic),*,) as SystemParam>::get_param(&mut context);
                self($(
                    params.$index
                ),*)
            }
        }
    };
}
impl_system_param_fn!((A, 0));
impl_system_param_fn!((A, 0)(B, 1));
impl_system_param_fn!((A, 0)(B, 1)(C, 2));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8));
impl_system_param_fn!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9));
