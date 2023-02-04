use crate::{app::App, event::Event};

pub struct SystemContext<'c> {
    pub app: &'c mut App,
    pub event: Option<Event>,
}
