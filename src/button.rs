use crate::system::System;

pub struct Button<D> {
    pub(crate) on_click: Box<dyn System<D>>,
}

impl<D> Button<D> {
    pub fn new() -> Self {
        Self {
            on_click: Box::new(|_: &D| {}),
        }
    }

    pub fn on_click(mut self, callback: impl System<D> + 'static) -> Self {
        self.on_click = Box::new(callback);
        self
    }
}
