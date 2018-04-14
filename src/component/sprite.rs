extern crate sdl2;

use sdl2::pixels::Color;

pub struct Sprite {
    pub color: Color,
    pub fill: bool,
    pub z_index: i32
}