use crate::access::Access;
use crate::container::Container;
use crate::system::SystemParamFunction;
use crate::system_param::SystemParam;

macro_rules! impl_system_param_fn {
    ($($param: ident),*) => {
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

        impl<F, $($param: SystemParam),*> SystemParamFunction<($($param),*,)> for F
        where
            F: Fn($($param),*) -> () + 'static,
        {
            fn initialize(access: &mut Access)
            where
                Self: Sized
            {
                <($($param),*,) as SystemParam>::initialize(access);
            }

            fn run(&mut self, container: &mut Container) {
                let _params = <($($param),*,) as SystemParam>::get_param(container);
                eprintln!("totally calling a function here");
            }
        }
    };
}
impl_system_param_fn!(P1);
impl_system_param_fn!(P1, P2);
