use std::{cell::RefCell, rc::Weak};

use crate::{children::Children, widget::Widget};

pub struct WidgetContext<'c> {
    pub parent: Weak<RefCell<dyn Widget>>,
    pub children: &'c mut Children,
}
