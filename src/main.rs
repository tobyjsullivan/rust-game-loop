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
use component::{
    ComponentManager,
    Motion,
    Sprite,
    Transform,
    Follow,
    Camera
};
use system::{Render, Movement, Tracking};

const SCREEN_HEIGHT: u32 = 640;
const SCREEN_WIDTH: u32 = 1200;
const WORLD_WIDTH: i32 = 50;
const WORLD_HEIGHT: i32 = 50;
const START_X: f32 = 1.0;
const START_Y: f32 = 1.0;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    let mut render = Render::new(&sdl_ctx, SCREEN_WIDTH, SCREEN_HEIGHT);
    let movement = Movement::new();
    let tracking = Tracking::new();

    let mut producer = EntityProducer::new();

    let mut sprites: ComponentManager<Sprite> = ComponentManager::new();
    let mut transforms: ComponentManager<Transform> = ComponentManager::new();
    let mut motions: ComponentManager<Motion> = ComponentManager::new();
    let mut followers: ComponentManager<Follow> = ComponentManager::new();
    let mut cameras: ComponentManager<Camera> = ComponentManager::new();

    match init_land_tiles(&mut producer, sprites, transforms) {
        (s, t) => {
            sprites = s;
            transforms = t;
        }
    }

    let player = producer.create();
    sprites = sprites.set(&player, Sprite{color: Color::RGB(255, 0, 0), fill: true});
    transforms = transforms.set(&player, Transform{x: START_X, y: START_Y});
    motions = motions.set(&player, Motion{velo_x: 1.0, velo_y: 1.0});

    let camera1 = producer.create();
    sprites = sprites.set(&camera1, Sprite{color: Color::RGB(255, 255, 255), fill: false});
    transforms = transforms.set(&camera1, Transform{x: START_X, y: START_Y});
    motions = motions.set(&camera1, Motion{velo_x: 0.0, velo_y: 0.0});
    followers = followers.set(&camera1, Follow{target: player.clone(), speed: 100.0});
    cameras = cameras.set(&camera1, Camera{view_width: 10.0, view_height: 10.0});

    let mut frame_count = 0;
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

        motions = tracking.apply(ticks, &followers, &transforms, motions);
        transforms = movement.apply(ticks, transforms, &motions);

        transforms.get(&player).map(|t| {
            println!("Player: {}x{}", t.x, t.y);
        });
        transforms.get(&camera1).map(|t| {
            println!("Camera: {}x{}", t.x, t.y);
        });
        render.render(ticks, &cameras, &sprites, &transforms);

        frame_count += 1;
        // if frame_count > 1 {
        //     break;
        // }
    }
}

fn create_land_tile(x: f32, y: f32, producer: &mut EntityProducer, sprites: ComponentManager<Sprite>, transforms: ComponentManager<Transform>) -> (ComponentManager<Sprite>, ComponentManager<Transform>) {
    let tile = producer.create();
    (
        sprites.set(&tile, Sprite{color: Color::RGB(0, 255, 0), fill: false}),
        transforms.set(&tile, Transform{ x, y })
    )
}

fn init_land_tiles(producer: &mut EntityProducer, sprites: ComponentManager<Sprite>, transforms: ComponentManager<Transform>) -> (ComponentManager<Sprite>, ComponentManager<Transform>) {
    let mut new_sprites = sprites;
    let mut new_transforms = transforms;
    for x in 0..WORLD_WIDTH {
        for y in 0..WORLD_HEIGHT {
            match create_land_tile(x as f32, y as f32, producer, new_sprites, new_transforms) {
                (s, t) => {
                    new_sprites = s;
                    new_transforms = t
                }
            }
        }
    }

    (new_sprites, new_transforms)
}
