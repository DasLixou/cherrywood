use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
};

use crate::{children::Children, system::BoxedDescribedSystem};

pub trait Widget {
    fn fetch_events(&mut self, event_type: TypeId) -> Vec<BoxedDescribedSystem>;

    fn parent(&mut self) -> BoxedWidget;

    fn children_mut(&mut self) -> Children;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type BoxedWidget = Rc<RefCell<dyn Widget>>;
