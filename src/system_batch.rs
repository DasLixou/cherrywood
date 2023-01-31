use crate::system::{BoxedDescribedSystem, DescribedSystem};

pub trait SystemBatch {
    const CAPACITY: usize;

    type IntoIter: Iterator<Item = BoxedDescribedSystem>;

    fn into_iter(self) -> Self::IntoIter;
}

impl SystemBatch for () {
    const CAPACITY: usize = 0;

    type IntoIter = std::iter::Empty<BoxedDescribedSystem>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl<S: DescribedSystem<()> + 'static> SystemBatch for S {
    const CAPACITY: usize = 1;

    type IntoIter = std::iter::Once<BoxedDescribedSystem>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(Box::new(self))
    }
}

macro_rules! method_arity {
    ( $($arg:tt)* ) => {
      { 0usize $(+ replace_expr!($arg 1usize))* }
    }
  }

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! impl_system_batch_tuple {
    ($(($generic:ident, $index:tt))+) => {
        impl<$($generic: DescribedSystem<()> + 'static),*> SystemBatch for ($($generic,)+) {
            const CAPACITY: usize = method_arity!($($generic)*);

            type IntoIter = std::array::IntoIter<BoxedDescribedSystem, { method_arity!($($generic)*) }>;

            fn into_iter(self) -> Self::IntoIter {
                ([$(
                    Box::new(self.$index)
                ),*] as [BoxedDescribedSystem; { method_arity!($($generic)*) }]).into_iter()
            }
        }
    };
}

impl_system_batch_tuple!((A, 0));
impl_system_batch_tuple!((A, 0)(B, 1));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8));
impl_system_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9));
