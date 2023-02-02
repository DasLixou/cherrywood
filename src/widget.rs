use std::{
    any::{Any, TypeId},
    rc::Rc,
    sync::Mutex,
};

use crate::system::BoxedDescribedSystem;

pub trait Widget {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn children_mut(&mut self) -> Vec<BoxedWidget>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type BoxedWidget = Rc<Mutex<dyn Widget>>;
