use crate::event::Event;

pub trait EventBatch {
    const CAPACITY: usize;

    type IntoIter: Iterator<Item = Event>;

    fn into_iter(self) -> Self::IntoIter;
}

impl EventBatch for () {
    const CAPACITY: usize = 0;

    type IntoIter = std::iter::Empty<Event>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl EventBatch for Event {
    const CAPACITY: usize = 1;

    type IntoIter = std::iter::Once<Event>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl EventBatch for Vec<Event> {
    const CAPACITY: usize = 0;

    type IntoIter = std::vec::IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        <Self as IntoIterator>::into_iter(self)
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
    ($(($str:path, $index:tt))+) => {
        impl EventBatch for ($($str,)+) {
            const CAPACITY: usize = method_arity!($($str)*);

            type IntoIter = std::array::IntoIter<Event, { method_arity!($($str)*) }>;

            fn into_iter(self) -> Self::IntoIter {
                ([$(
                    self.$index
                ),*] as [Event; { method_arity!($($str)*) }]).into_iter()
            }
        }
    };
}

impl_widget_batch_tuple!((Event, 0));
impl_widget_batch_tuple!((Event, 0)(Event, 1));
impl_widget_batch_tuple!((Event, 0)(Event, 1)(Event, 2));
impl_widget_batch_tuple!((Event, 0)(Event, 1)(Event, 2)(Event, 3));
impl_widget_batch_tuple!((Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4));
impl_widget_batch_tuple!((Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4)(Event, 5));
impl_widget_batch_tuple!((Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4)(Event, 5)(Event, 6));
impl_widget_batch_tuple!(
    (Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4)(Event, 5)(Event, 6)(Event, 7)
);
impl_widget_batch_tuple!(
    (Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4)(Event, 5)(Event, 6)(Event, 7)(Event, 8)
);
impl_widget_batch_tuple!(
    (Event, 0)(Event, 1)(Event, 2)(Event, 3)(Event, 4)(Event, 5)(Event, 6)(Event, 7)(Event, 8)(
        Event, 9
    )
);
