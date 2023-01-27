use bevy_ptr::OwningPtr;

// PHANTOM is used for working generic impls like `impl<F, In> System<In> for F where F: FnMut(&In)`
pub trait IntoDescribedSystem<PHANTOM = ()> {
    fn into_described(self) -> DescribedSystem;
}

impl IntoDescribedSystem for () {
    fn into_described(self) -> DescribedSystem {
        OwningPtr::make(self, |ptr| DescribedSystem {
            system: ptr.as_ptr(),
        })
    }
}

macro_rules! impl_system_trait {
    ($($param: ident),*) => {
        impl<Func, $($param),*> IntoDescribedSystem<($($param),*)> for Func
        where
            Func: FnMut($($param),*),
        {
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
