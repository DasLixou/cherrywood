use bevy_ptr::OwningPtr;

// PHANTOM is used for working generic impls like `impl<F, In> System<In> for F where F: FnMut(&In)`
pub trait System<PHANTOM = ()> {
    fn run(&mut self);

    fn into_described(self) -> DescribedSystem;
}

impl System for () {
    fn run(&mut self) {}

    fn into_described(self) -> DescribedSystem {
        OwningPtr::make(self, |ptr| DescribedSystem {
            system: ptr.as_ptr(),
        })
    }
}

macro_rules! impl_system_trait {
    ($($param: ident),*) => {
        impl<Func, $($param),*> System<($($param),*)> for Func
        where
            Func: FnMut($($param),*),
        {
            fn run(&mut self) {
                fn call_inner(mut f: impl FnMut($($param),*)) {
                    f();
                }
                call_inner(self)
            }

            fn into_described(self) -> DescribedSystem {
                OwningPtr::make(self, |ptr| DescribedSystem {
                    system: ptr.as_ptr(),
                })
            }
        }
    };
}

impl_system_trait!();

pub struct DescribedSystem {
    system: *const u8,
}

impl DescribedSystem {
    pub fn run(&mut self) {
        todo!()
    }
}
