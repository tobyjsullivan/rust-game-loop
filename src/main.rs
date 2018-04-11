#![feature(duration_extras)]

#[macro_use]
extern crate im;
extern crate sdl2;

use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::{Duration, Instant};

mod sprite;
mod entity;
mod render;
mod transform;
mod motion;
mod movement;

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 1200;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    let mut render = render::Render::new(&sdl_ctx, SCREEN_WIDTH, SCREEN_HEIGHT);

    let e1 = entity::Entity{id: 1};
    let e2 = entity::Entity{id: 2};
    let e3 = entity::Entity{id: 3};

    let mut sprites = hashmap!{
        entity::Entity{ .. e1 } => sprite::Sprite{width: 32, height: 32, color: Color::RGB(255, 0, 0)},
        entity::Entity{ .. e2 } => sprite::Sprite{width: 32, height: 32, color: Color::RGB(0, 255, 0)},
        entity::Entity{ .. e3 } => sprite::Sprite{width: 32, height: 32, color: Color::RGB(0, 0, 255)}
    };

    let mut transforms = hashmap!{
        entity::Entity{ .. e1 } => transform::Transform{x: 3, y: 12},
        entity::Entity{ .. e2 } => transform::Transform{x: 5, y: 2},
        entity::Entity{ .. e3 } => transform::Transform{x: 16, y: 8}
    };

    let mut motions = hashmap!{
        entity::Entity{ .. e1 } => motion::Motion{acc_x: 0.1, acc_y: 0.1},
        entity::Entity{ .. e2 } => motion::Motion{acc_x: 0.0, acc_y: 0.0},
        entity::Entity{ .. e3 } => motion::Motion{acc_x: 0.0, acc_y: 0.0}
    };

    let mut last_tick = Instant::now();
    'main: loop {
        let ticks = last_tick.elapsed().subsec_millis();
        last_tick = Instant::now();
        println!("Ticks elapsed: {}", ticks);

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        transforms = movement::movement(ticks, transforms, &motions);
        render.render(ticks, &sprites, &transforms);
    }
}