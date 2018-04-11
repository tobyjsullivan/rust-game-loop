use im::HashMap;
use entity::Entity;
use transform::Transform;
use motion::Motion;
use std::sync::Arc;

pub fn movement(transforms: HashMap<Entity, Transform>, motions: &HashMap<Entity, Motion>) -> HashMap<Entity, Transform> {
    let mut new_transforms = transforms.clone();
    for entity in transforms.keys() {
        new_transforms = new_transforms.update_with_key(&entity, |e, t| {
            apply_movement((Some(t), motions.get(&e)))
        });
    }
    new_transforms.clone()
}

fn apply_movement(input: (Option<Arc<Transform>>, Option<Arc<Motion>>)) -> Option<Arc<Transform>> {
    match input {
        (Some(t), Some(m)) => {
            Some(Arc::new(Transform{
                x: t.x + m.acc_x,
                y: t.y + m.acc_y
            }))
        },
        (Some(t), None) => Some(t),
        (None, _) => None
    }
}