#![feature(duration_extras)]

extern crate im;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Instant;

mod entity;
mod component;
mod system;

use entity::EntityProducer;
use component::ComponentManager;
use component::Motion;
use component::Sprite;
use component::Transform;
use system::Render;
use system::Movement;

const SCREEN_HEIGHT: u32 = 640;
const SCREEN_WIDTH: u32 = 1200;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    let mut render = Render::new(&sdl_ctx, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut movement = Movement::new();

    let mut producer = EntityProducer::new();

    let tile00 = producer.create();
    let tile01 = producer.create();
    let tile02 = producer.create();
    let tile10 = producer.create();
    let tile11 = producer.create();
    let tile12 = producer.create();
    let tile20 = producer.create();
    let tile21 = producer.create();
    let tile22 = producer.create();

    let player = producer.create();

    let mut sprites: ComponentManager<Sprite> = ComponentManager::new();
    sprites = sprites.set(&tile00, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile01, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile02, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile10, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile11, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile12, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile20, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile21, Sprite{color: Color::RGB(0, 255, 0)});
    sprites = sprites.set(&tile22, Sprite{color: Color::RGB(0, 255, 0)});

    sprites = sprites.set(&player, Sprite{color: Color::RGB(255, 0, 0)});

    let mut transforms: ComponentManager<Transform> = ComponentManager::new();
    transforms = transforms.set(&tile00, Transform{x: 0.0, y: 0.0});
    transforms = transforms.set(&tile01, Transform{x: 1.0, y: 0.0});
    transforms = transforms.set(&tile02, Transform{x: 2.0, y: 0.0});
    transforms = transforms.set(&tile10, Transform{x: 0.0, y: 1.0});
    transforms = transforms.set(&tile11, Transform{x: 1.0, y: 1.0});
    transforms = transforms.set(&tile12, Transform{x: 2.0, y: 1.0});
    transforms = transforms.set(&tile20, Transform{x: 0.0, y: 2.0});
    transforms = transforms.set(&tile21, Transform{x: 1.0, y: 2.0});
    transforms = transforms.set(&tile22, Transform{x: 2.0, y: 2.0});

    transforms = transforms.set(&player, Transform{x: 1.0, y: 1.0});

    let mut motions: ComponentManager<Motion> = ComponentManager::new();
    motions = motions.set(&player, Motion{velo_x: 2.0, velo_y: 2.0});

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