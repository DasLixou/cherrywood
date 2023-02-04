use crate::{app::App, ptr::holding_ptr::HoldingPtr};

pub struct SystemContext<'c> {
    pub app: &'c mut App,
    pub event: Option<HoldingPtr>,
}
