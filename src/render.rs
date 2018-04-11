use im::HashMap;

use entity::Entity;
use sprite::Sprite;
use transform::Transform;

pub fn render(sprites: &HashMap<Entity, Sprite>, transforms: &HashMap<Entity, Transform>) {
    for entity in sprites.keys() {
        match (sprites.get(&entity), transforms.get(&entity)) {
            (Some(s), Some(t)) => {
                println!("Rendering: {} at {}x{}", s.img, t.x, t.y)
            },
            (_, _) => ()
        };

    }
}