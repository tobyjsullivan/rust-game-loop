
extern crate sdl2;

use sdl2::pixels::Color;
use im::shared::Shared;

#[derive(Debug, Clone)]
pub struct Motion {
    pub velo_x: f32,
    pub velo_y: f32
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub color: Color
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub x: f32,
    pub y: f32
}
