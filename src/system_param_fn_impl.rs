use crate::access::Access;
use crate::container::Container;
use crate::system::SystemParamFunction;
use crate::system_param::SystemParam;

impl SystemParam for () {
    type Param<'c> = ();

    fn initialize(_access: &mut Access) {}

    fn get_param<'c>(_container: &'c mut Container) -> Self::Param<'c> {
        ()
    }
}

impl<F> SystemParamFunction<()> for F
where
    F: Fn() -> () + 'static,
{
    fn initialize(_access: &mut Access)
    where
        Self: Sized,
    {
    }

    fn run(&mut self, _container: &mut Container) {
        self();
    }
}

macro_rules! impl_system_param_fn {
    ($(($param: ident, $index:tt)),*) => {
        impl<$($param: SystemParam),*> SystemParam for ($($param),*,) {
            type Param<'c> = ($($param::Param<'c>),*,);

            fn initialize(access: &mut Access) {
                $(
                    <$param as SystemParam>::initialize(access);
                )*
            }

            fn get_param<'c>(container: &'c mut Container) -> Self::Param<'c> {
                ($(
                    <$param as SystemParam>::get_param(unsafe {
                        // SAFETY: we already checked for conflicts in `initialize`
                        &mut *((&mut *container) as *mut Container)
                    })
                ),*,)
            }
        }

        impl<Func, $($param: SystemParam),*> SystemParamFunction<($($param),*,)> for Func
        where
            Func: Fn($($param),*) -> () + Fn($($param::Param<'_>),*) -> () + 'static,
        {
            fn initialize(access: &mut Access)
            where
                Self: Sized
            {
                <($($param),*,) as SystemParam>::initialize(access);
            }

            fn run(&mut self, container: &mut Container) {
                let params = <($($param),*,) as SystemParam>::get_param(container);
                self($(
                    params.$index
                ),*);
            }
        }
    };
}
impl_system_param_fn!((A, 0));
impl_system_param_fn!((A, 0), (B, 1));
impl_system_param_fn!((A, 0), (B, 1), (C, 2));
impl_system_param_fn!((A, 0), (B, 1), (C, 2), (D, 3));
impl_system_param_fn!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_system_param_fn!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_system_param_fn!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
