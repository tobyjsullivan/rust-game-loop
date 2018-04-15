#![feature(duration_extras)]

extern crate im;
extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use im::HashSet;
use rand::{Rng, thread_rng};

mod entity;
mod component;
mod system;
mod control;
mod map;

use entity::EntityProducer;
use component::{
    ComponentManager,
    Sprite,
    Transform,
    Camera,
    Joystick,
    Walk
};
use system::{
    Render,
    Controller,
    Walking
};
use control::Control;
use map::Tile;

const SCREEN_HEIGHT: u32 = 640;
const SCREEN_WIDTH: u32 = 1200;
const WORLD_WIDTH: i32 = 20;
const WORLD_HEIGHT: i32 = 20;
const START_X: f64 = 1.0;
const START_Y: f64 = 1.0;
const PLAYER_SPEED: f64 = 8.0;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    let mut render = Render::new(&sdl_ctx, SCREEN_WIDTH, SCREEN_HEIGHT, WORLD_WIDTH as f64, WORLD_HEIGHT as f64);
    let controller = Controller::new(PLAYER_SPEED);
    let walking = Walking::new();

    let mut producer = EntityProducer::new();

    let mut sprites: ComponentManager<Sprite> = ComponentManager::new();
    let mut transforms: ComponentManager<Transform> = ComponentManager::new();
    let mut cameras: ComponentManager<Camera> = ComponentManager::new();
    let mut joysticks: ComponentManager<Joystick> = ComponentManager::new();
    let mut walkers: ComponentManager<Walk> = ComponentManager::new();

    match init_land_tiles(&mut producer, sprites, transforms) {
        (s, t) => {
            sprites = s;
            transforms = t;
        }
    }

    let player = producer.create();
    sprites = sprites.set(&player, Sprite{color: Color::RGB(255, 0, 0), fill: true, z_index: 1});
    transforms = transforms.set(&player, Transform{x: START_X, y: START_Y});
    joysticks = joysticks.set(&player, Joystick{});
    // Since we want to make the camera follow the player, we just make the player the camera. *brain explodes*
    cameras = cameras.set(&player, Camera{view_width: 10.0, view_height: 10.0});

    let mut frame_count = 0;
    let mut last_tick = Instant::now();
    let mut controls: HashSet<Control> = HashSet::new();
    'main: loop {
        let ticks = last_tick.elapsed().subsec_millis();
        last_tick = Instant::now();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main,
                _ => {}
            }

            controls = Control::mutate_controls(controls, event);
        }

        walkers = controller.apply(ticks, &controls, &joysticks, walkers, &transforms);
        match walking.apply(ticks, walkers, transforms) {
            (w, t) => {
                walkers = w;
                transforms = t;
            }
        }

        render.render(ticks, &cameras, &sprites, &transforms);

        frame_count += 1;
        // if frame_count > 1 {
        //     break;
        // }
    }
}

fn create_land_tile(x: f64, y: f64, tile: &Tile, producer: &mut EntityProducer, sprites: ComponentManager<Sprite>, transforms: ComponentManager<Transform>) -> (ComponentManager<Sprite>, ComponentManager<Transform>) {
    let entity = producer.create();
    (
        sprites.set(&entity, Sprite{color: Tile::color(tile), fill: true, z_index: 0}),
        transforms.set(&entity, Transform{ x, y })
    )
}

fn init_land_tiles(producer: &mut EntityProducer, sprites: ComponentManager<Sprite>, transforms: ComponentManager<Transform>) -> (ComponentManager<Sprite>, ComponentManager<Transform>) {
    let mut new_sprites = sprites;
    let mut new_transforms = transforms;
    let mut prev_tile: Tile = Tile::Grass;
    for x in 0..WORLD_WIDTH {
        for y in 0..WORLD_HEIGHT {
            let next_tile = next_tile(&prev_tile);
            match create_land_tile(x as f64, y as f64, &next_tile, producer, new_sprites, new_transforms) {
                (s, t) => {
                    new_sprites = s;
                    new_transforms = t
                }
            }
            prev_tile = next_tile;
        }
    }

    (new_sprites, new_transforms)
}

fn next_tile(prev: &Tile) -> Tile {
    let mut rng = thread_rng();
    if rng.gen::<f32>() < 0.7 {
        Tile::Grass
    } else {
        Tile::Water
    }
}
