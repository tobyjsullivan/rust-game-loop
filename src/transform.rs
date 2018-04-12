use im::shared::Shared;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Transform {
    pub x: i32,
    pub y: i32
}

impl Transform {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Transform{ x: pos_x, y: pos_y }
    }
}
