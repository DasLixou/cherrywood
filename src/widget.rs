use std::any::TypeId;

use crate::{as_any::AsAny, system::BoxedDescribedSystem, widgets::WidgetId};

pub trait Widget: AsAny {
    fn fetch_events(&self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn id(&self) -> WidgetId;
}

pub type BoxedWidget = Box<dyn Widget>;
