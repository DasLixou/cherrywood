use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::widget::Widget;

#[derive(Clone)]
pub struct WidgetHandle<W: Widget> {
    pub(crate) data: Rc<RefCell<W>>,
    phantom: PhantomData<W>,
}

impl<W: Widget> WidgetHandle<W> {
    pub fn new(widget: W) -> Self {
        Self {
            data: Rc::new(RefCell::new(widget)),
            phantom: PhantomData,
        }
    }

    pub fn finish(&mut self) -> Rc<RefCell<W>> {
        self.data.clone()
    }
}
