use crate::button::Button;

pub struct Container<D> {
    main: Button<D>,
    data: D,
}

impl<D> Container<D> {
    pub fn new(main: Button<D>, data: D) -> Self {
        Self { main, data }
    }

    pub fn click(&mut self) {
        self.main.on_click.run(&self.data)
    }
}
