use crate::{batch::widget::WidgetBatch, widget::BoxedWidget};

#[derive(Clone)]
pub struct Children {
    children: Vec<BoxedWidget>,
}

impl Children {
    pub const NONE: Self = Children::new();

    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn extend_batch<B: WidgetBatch>(&mut self, batch: B) {
        self.children.reserve(B::CAPACITY);
        self.children.extend(batch.into_iter());
    }

    pub fn iter(&mut self) -> impl Iterator<Item = BoxedWidget> + '_ {
        self.children.iter().map(|rc| rc.clone())
    }
}
