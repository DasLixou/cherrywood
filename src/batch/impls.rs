use crate::{
    impl_batch,
    system::{BoxedDescribedSystem, DescribedSystem},
    widget::{BoxedWidget, Widget},
};

impl_batch!(WidgetBatch, Widget, BoxedWidget);
impl_batch!(SystemBatch, DescribedSystem<()>, BoxedDescribedSystem);
