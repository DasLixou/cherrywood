use cherrywood_functional::System;

pub struct Button {
    on_click: Box<dyn System>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            on_click: Box::new(()),
        }
    }

    pub fn on_click(mut self, callback: impl System + 'static) -> Self {
        self.on_click = Box::new(callback);
        self
    }

    pub fn click(&mut self) {
        self.on_click.run()
    }
}
