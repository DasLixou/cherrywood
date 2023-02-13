use dyn_clone::DynClone;

use crate::{as_any::AsAny, math::point::Point, widgets::WidgetId};

#[derive(Clone)]
pub struct Event {
    pub(crate) message: Box<dyn EventMessage>,
    pub(crate) kind: EventKind,
}

impl Event {
    pub fn new<E: EventMessage + 'static>(message: E, kind: EventKind) -> Self {
        Self {
            message: Box::new(message),
            kind,
        }
    }
}

#[derive(Clone)]
pub enum EventKind {
    /// **The event will appear in the root widget and then pass trough it's children.**
    ///
    /// Gets converted into [`EventKind::FallingFrom`] with the root widget.
    Root,
    /// **The event will appear in this widget and then pass trough it's children.**
    ///
    /// Gets converted into [`EventKind::FallingFrom`] with the current widget.
    Falling,
    Climbing,
    /// **The event will only appear in this widget.**
    ///
    /// Gets converted into [`EventKind::BubbleIn`] with the current widget.
    Bubble,
    /// **The event will appear in the given widget and then pass trough it's children.**
    FallingFrom(WidgetId),
    /// **The event will only appear in the given widget.**
    BubbleIn(WidgetId),
}

pub trait EventMessage: DynClone + AsAny {}

dyn_clone::clone_trait_object!(EventMessage);

#[derive(Clone)]
pub struct PointerClick(pub Point);
impl EventMessage for PointerClick {}

#[derive(Clone)]
pub struct OnClick(pub Point);
impl EventMessage for OnClick {}
