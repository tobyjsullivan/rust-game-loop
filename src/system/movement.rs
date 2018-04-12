use im::HashMap;
use entity::Entity;
use component::Transform;
use component::Motion;
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
            let move_portion = 1000.0 / ticks as f32;
            let x_dist = m.velo_x / move_portion;
            if (x_dist > 0.0) {
                println!("Moving {}", x_dist);
            }
            pos_x += x_dist;
            pos_y += m.velo_y / move_portion;

            (
                Some(Transform{
                    x: pos_x,
                    y: pos_y,
                    .. *t.deref()
                }),
                Some((*m.deref()).clone())
            )
        },
        (_, _) => (None, None)
    }
}