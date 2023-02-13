use std::any::{Any, TypeId};

use crate::{system::BoxedDescribedSystem, widgets::WidgetId};

pub trait Widget {
    fn fetch_events(&self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn id(&self) -> WidgetId;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type BoxedWidget = Box<dyn Widget>;
