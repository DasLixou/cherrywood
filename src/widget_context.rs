use crate::app::App;

#[derive(Clone, Copy)]
pub struct WidgetContext {
    pub app: *mut App,
}
