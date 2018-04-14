#![feature(duration_extras)]

#[macro_use]
extern crate im;
extern crate sdl2;

use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::{Duration, Instant};

mod entity;
mod component;
mod system;

use entity::Entity;
use component::ComponentManager;
use component::Motion;
use component::Sprite;
use component::Transform;
use system::Render;
use system::Movement;

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 1200;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    let mut render = Render::new(&sdl_ctx, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut movement = Movement::new();

    let e1 = entity::Entity{id: 1};
    let e2 = entity::Entity{id: 2};
    let e3 = entity::Entity{id: 3};

    let mut sprites: ComponentManager<Sprite> = ComponentManager::new();
    sprites = sprites.set(&e1, Sprite{width: 32, height: 32, color: Color::RGB(255, 0, 0)});
    sprites = sprites.set(&e2, Sprite{width: 32, height: 32, color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&e3, Sprite{width: 32, height: 32, color: Color::RGB(0, 0, 255)});

    let mut transforms: ComponentManager<Transform> = ComponentManager::new();
    transforms = transforms.set(&e1, Transform{x: 3.0, y: 12.0});
    transforms = transforms.set(&e2, Transform{x: 5.0, y: 2.0});
    transforms = transforms.set(&e3, Transform{x: 16.0, y: 8.0});

    let mut motions: ComponentManager<Motion> = ComponentManager::new();
    motions = motions.set(&e1, Motion{velo_x: 100.0, velo_y: 100.0});
    motions = motions.set(&e2, Motion{velo_x: 0.0, velo_y: 0.0});
    motions = motions.set(&e3, Motion{velo_x: 0.0, velo_y: 0.0});

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

        transforms = movement.apply(ticks, transforms, &motions);
        render.render(ticks, &sprites, &transforms);
    }
}