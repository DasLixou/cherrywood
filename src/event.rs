use crate::math::point::Point;

pub trait Event {}

pub struct PointerClick(pub Point);
impl Event for PointerClick {}
