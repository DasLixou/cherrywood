use crate::{app::App, event::Event, widgets::WidgetId};

pub struct SystemContext<'c> {
    pub app: &'c mut App,
    pub event: Event,
    pub widget: WidgetId,
}
