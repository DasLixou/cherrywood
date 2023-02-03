use crate::math::point::Point;

pub struct Event<E: EventMessage> {
    pub(crate) message: E,
    kind: EventKind,
}

impl<E: EventMessage> Event<E> {
    pub fn new(message: E, kind: EventKind) -> Self {
        Self { message, kind }
    }
}

impl<E: EventMessage> Clone for Event<E> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            kind: self.kind.clone(),
        }
    }
}

#[derive(Clone)]
pub enum EventKind {
    Falling,
    Climbing,
    Bubble,
}

pub trait EventMessage {
    fn clone(&self) -> Self
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct PointerClick(pub Point);
impl EventMessage for PointerClick {
    fn clone(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(&self)
    }
}

#[derive(Clone)]
pub struct OnClick(pub Point);
impl EventMessage for OnClick {
    fn clone(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(&self)
    }
}
