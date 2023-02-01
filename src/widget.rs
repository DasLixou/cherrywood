use std::any::{Any, TypeId};

use crate::holding_ptr::HoldingPtr;

pub trait Widget {
    fn dispatch_event(&mut self, t: TypeId, ptr: HoldingPtr) -> Option<HoldingPtr>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
