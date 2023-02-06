use std::{cell::RefCell, rc::Rc};

use crate::widget::Widget;

pub trait WidgetBatch {
    const CAPACITY: usize;

    type IntoIter: Iterator<Item = Rc<RefCell<dyn Widget>>>;

    fn into_iter(self) -> Self::IntoIter;
}

impl WidgetBatch for () {
    const CAPACITY: usize = 0;

    type IntoIter = std::iter::Empty<Rc<RefCell<dyn Widget>>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl<W: Widget + 'static> WidgetBatch for Rc<RefCell<W>> {
    const CAPACITY: usize = 1;

    type IntoIter = std::iter::Once<Rc<RefCell<dyn Widget>>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
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

macro_rules! impl_widget_batch_tuple {
    ($(($generic:ident, $index:tt))+) => {
        impl<$($generic: Widget + 'static),*> WidgetBatch for ($(Rc<RefCell<$generic>>,)+) { // TODO: this won't work because of `&mut` and &mut will cause a drop of temp value
            const CAPACITY: usize = method_arity!($($generic)*);

            type IntoIter = std::array::IntoIter<Rc<RefCell<dyn Widget>>, { method_arity!($($generic)*) }>;

            fn into_iter(self) -> Self::IntoIter {
                ([$(
                    self.$index
                ),*] as [Rc<RefCell<dyn Widget>>; { method_arity!($($generic)*) }]).into_iter()
            }
        }
    };
}

impl_widget_batch_tuple!((A, 0));
impl_widget_batch_tuple!((A, 0)(B, 1));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8));
impl_widget_batch_tuple!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9));
