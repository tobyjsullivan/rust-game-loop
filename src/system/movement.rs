use component::Transform;
use component::Motion;
use component::ComponentManager;
use std::sync::Arc;
use std::ops::Deref;

pub struct Movement {}

impl Movement {
    pub fn new() -> Self {
        Movement{}
    }

    pub fn apply(&self, ticks: u32, transforms: ComponentManager<Transform>, motions: &ComponentManager<Motion>) -> ComponentManager<Transform> {
        let transforms_keys = transforms.keys();
        let mut new_transforms = transforms;
        for entity in transforms_keys {
            match apply_movement(ticks, new_transforms.get(&entity), motions.get(&entity)) {
                Some(t) => {
                    new_transforms = new_transforms.set(&entity, t.clone());
                }
                None => {}
            }
        }
        new_transforms
    }
}


fn apply_movement(ticks: u32, transform: Option<Arc<Transform>>, motion: Option<Arc<Motion>>) -> Option<Transform> {
    match (transform, motion) {
        (Some(t), Some(m)) => {
            let mut pos_x = t.x;
            let mut pos_y = t.y;
            let move_portion = 1000.0 / ticks as f32;
            let x_dist = m.velo_x / move_portion;
            pos_x += x_dist;
            pos_y += m.velo_y / move_portion;

            Some(Transform{
                x: pos_x,
                y: pos_y,
                .. *t.deref()
            })
        },
        (_, _) => None
    }
}