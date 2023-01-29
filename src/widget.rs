use std::any::Any;

use slotmap::new_key_type;

new_key_type! {
    pub struct WidgetId;
}

pub trait Widget {
    fn id(&self) -> WidgetId;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
