use std::any::{Any, TypeId};

use crate::system::BoxedDescribedSystem;

pub trait Widget {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
