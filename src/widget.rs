use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
};

use crate::system::BoxedDescribedSystem;

// TODO: add parent, because with Rc we now can ^^
pub trait Widget {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn children_mut(&mut self) -> Vec<BoxedWidget>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type BoxedWidget = Rc<RefCell<dyn Widget>>;
