#[macro_use]
extern crate im;

use std::thread;
use std::time::Duration;

mod sprite;
mod entity;
mod render;
mod transform;
mod motion;
mod movement;

fn main() {
    let e1 = entity::Entity{id: 1};
    let e2 = entity::Entity{id: 2};
    let e3 = entity::Entity{id: 3};

    let mut sprites = hashmap!{
        entity::Entity{ .. e1 } => sprite::Sprite{img: "entity1"},
        entity::Entity{ .. e2 } => sprite::Sprite{img: "entity2"},
        entity::Entity{ .. e3 } => sprite::Sprite{img: "entity3"}
    };

    let mut transforms = hashmap!{
        entity::Entity{ .. e1 } => transform::Transform{x: 3, y: 12},
        entity::Entity{ .. e2 } => transform::Transform{x: 5, y: 2},
        entity::Entity{ .. e3 } => transform::Transform{x: 16, y: 8}
    };

    let mut motions = hashmap!{
        entity::Entity{ .. e1 } => motion::Motion{acc_x: 0, acc_y: 0},
        entity::Entity{ .. e2 } => motion::Motion{acc_x: 0, acc_y: 0},
        entity::Entity{ .. e3 } => motion::Motion{acc_x: 0, acc_y: 0}
    };

    loop {
        transforms = movement::movement(transforms, &motions);
        render::render(&sprites, &transforms);

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}