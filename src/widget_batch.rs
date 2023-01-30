use crate::widget::Widget;

pub trait WidgetBatch {
    type IntoIter: Iterator<Item = Box<dyn Widget>>;

    fn into_iter(self) -> Self::IntoIter;
}

impl<W: Widget + 'static> WidgetBatch for W {
    type IntoIter = std::iter::Once<Box<dyn Widget>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(Box::new(self))
    }
}
