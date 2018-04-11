use im::HashMap;
use entity::Entity;
use transform::Transform;
use motion::Motion;
use std::sync::Arc;

pub fn movement(ticks: u32, transforms: HashMap<Entity, Transform>, motions: &HashMap<Entity, Motion>) -> HashMap<Entity, Transform> {
    let mut new_transforms = transforms.clone();
    for entity in transforms.keys() {
        new_transforms = new_transforms.update_with_key(&entity, |e, t| {
            apply_movement(ticks, t, motions.get(&e))
        });
    }
    new_transforms.clone()
}

fn apply_movement(ticks: u32, transform: Arc<Transform>, motion: Option<Arc<Motion>>) -> Option<Arc<Transform>> {
    match (transform, motion) {
        (t, Some(m)) => {
            Some(Arc::new(Transform{
                x: t.x + (m.acc_x * ticks as f32) as i32,
                y: t.y + (m.acc_y * ticks as f32) as i32
            }))
        },
        (t, None) => Some(t)
    }
}