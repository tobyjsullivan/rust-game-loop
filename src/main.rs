#[macro_use]
extern crate im;

use im::Vector;

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

    let sprites = hashmap!{
        &e1 => &sprite::Sprite{img: "entity1"},
        &e2 => &sprite::Sprite{img: "entity2"},
        &e3 => &sprite::Sprite{img: "entity3"}
    };

    let transforms = hashmap!{
        &e1 => &transform::Transform{x: 3, y: 12},
        &e2 => &transform::Transform{x: 5, y: 2},
        &e3 => &transform::Transform{x: 16, y: 8}
    };

    let motions = hashmap!{
        &e1 => &motion::Motion{accX: 1, accY: 2}
    };

    let transforms = movement::movement(transforms, &motions);
    render::render(&sprites, &transforms);
}