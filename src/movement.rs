use im::HashMap;
use entity::Entity;
use transform::Transform;
use motion::Motion;
use std::sync::Arc;
use std::ops::Deref;

pub fn movement(ticks: u32, transforms: HashMap<Entity, Transform>, motions: HashMap<Entity, Motion>) -> (HashMap<Entity, Transform>, HashMap<Entity, Motion>) {
    let mut new_transforms = transforms.clone();
    let mut new_motions = motions.clone();
    for entity in transforms.keys() {
        match apply_movement(ticks, transforms.get(&entity), motions.get(&entity)) {
            (Some(t), Some(m)) => {
                new_motions = new_motions.update(&entity, |_| { Some(Arc::new(m.clone())) });
                new_transforms = new_transforms.update(&entity, |_| { Some(Arc::new(t.clone())) });
            }
            (_, _) => {}
        }
    }
    (new_transforms.clone(), new_motions.clone())
}

fn apply_movement(ticks: u32, transform: Option<Arc<Transform>>, motion: Option<Arc<Motion>>) -> (Option<Transform>, Option<Motion>) {
    match (transform, motion) {
        (Some(t), Some(m)) => {
            let mut pos_x = t.x;
            let mut pos_y = t.y;
            let mut next_move = m.next_move;
            if (next_move <= 0) {
                pos_x += m.velo_x;
                pos_y += m.velo_y;
                next_move += m.move_rate;
                println!("Moved! Next move: {}", next_move);
            } else {
                next_move -= ticks as i32;
                println!("Next move: {}", next_move);
            }

            (
                Some(Transform{
                    x: pos_x,
                    y: pos_y,
                    .. *t.deref()
                }),
                Some(Motion {
                    next_move: next_move,
                    .. *m.deref()
                })
            )
        },
        (_, _) => (None, None)
    }
}