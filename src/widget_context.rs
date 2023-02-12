use std::{cell::RefCell, marker::PhantomData, rc::Weak};

use crate::widget::Widget;

pub struct WidgetContext<'c> {
    pub parent: Weak<RefCell<dyn Widget>>,
    pub phantom: PhantomData<&'c ()>, // TODO: Do we need it?
}
