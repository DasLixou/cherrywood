use dyn_clone::DynClone;

use crate::{as_any::AsAny, math::point::Point};

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
    Root,
    Falling,
    Climbing,
    Bubble,
}

pub trait EventMessage: DynClone + AsAny {}

dyn_clone::clone_trait_object!(EventMessage);

#[derive(Clone)]
pub struct PointerClick(pub Point);
impl EventMessage for PointerClick {}

#[derive(Clone)]
pub struct OnClick(pub Point);
impl EventMessage for OnClick {}
