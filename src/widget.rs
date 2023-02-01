use std::any::{Any, TypeId};

use crate::{app::App, holding_ptr::HoldingPtr};

pub trait Widget {
    fn dispatch_event(&mut self, app: &mut App, t: TypeId, ptr: HoldingPtr) -> Option<HoldingPtr>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
