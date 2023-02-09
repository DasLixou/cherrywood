use crate::{app::App, event::Event, widget::BoxedWidget};

pub struct SystemContext<'c> {
    pub app: &'c mut App,
    pub event: Event,
    pub widget: &'c BoxedWidget,
}
