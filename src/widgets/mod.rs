use slotmap::new_key_type;

pub mod button;
pub mod label;
pub mod stack;

new_key_type! {
    pub struct WidgetId;
}
