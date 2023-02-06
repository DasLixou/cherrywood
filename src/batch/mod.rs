mod impls;
pub use impls::*;

pub mod event;
pub mod widget;

#[macro_export]
macro_rules! method_arity {
    ($($arg:tt)*) => {
        { 0usize $(+ $crate::replace_expr!($arg 1usize))* }
    }
}

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! impl_custom_batch_tuple {
    ($name:ident, $trait:path, $ret:ty, $(($generic:ident, $index:tt))+) => {
        impl<$($generic: $trait + 'static),*> $name for ($($generic,)+) {
            const CAPACITY: usize = $crate::method_arity!($($generic)*);

            type IntoIter = std::array::IntoIter<$ret, { $crate::method_arity!($($generic)*) }>;

            fn into_iter(self) -> Self::IntoIter {
                ([$(
                    std::rc::Rc::new(std::cell::RefCell::new(self.$index))
                ),*] as [$ret; { $crate::method_arity!($($generic)*) }]).into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_batch {
    ($name:ident, $trait:path, $ret:ty) => {
        pub trait $name {
            const CAPACITY: usize;

            type IntoIter: Iterator<Item = $ret>;

            fn into_iter(self) -> Self::IntoIter;
        }

        impl $name for () {
            const CAPACITY: usize = 0;

            type IntoIter = std::iter::Empty<$ret>;

            fn into_iter(self) -> Self::IntoIter {
                std::iter::empty()
            }
        }

        impl<W: $trait + 'static> $name for W {
            const CAPACITY: usize = 1;

            type IntoIter = std::iter::Once<$ret>;

            fn into_iter(self) -> Self::IntoIter {
                std::iter::once(std::rc::Rc::new(std::cell::RefCell::new(self)))
            }
        }

        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0));
        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0)(B, 1));
        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0)(B, 1)(C, 2));
        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0)(B, 1)(C, 2)(D, 3));
        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
        $crate::impl_custom_batch_tuple!($name, $trait, $ret, (A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
        $crate::impl_custom_batch_tuple!(
            $name,
            $trait,
            $ret,
            (A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)
        );
        $crate::impl_custom_batch_tuple!(
            $name,
            $trait,
            $ret,
            (A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)
        );
        $crate::impl_custom_batch_tuple!(
            $name,
            $trait,
            $ret,
            (A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)
        );
        $crate::impl_custom_batch_tuple!(
            $name,
            $trait,
            $ret,
            (A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9)
        );
    };
}
