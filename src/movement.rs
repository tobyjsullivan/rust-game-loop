use im::HashMap;
use entity::Entity;
use transform::Transform;
use motion::Motion;
use std::sync::Arc;

pub fn movement<'a>(transforms: HashMap<&'a Entity, &'a Transform>, motions: &HashMap<&'a Entity, &'a Motion>) -> HashMap<&'a Entity, &'a Transform> {
    let mut newTransforms = transforms.clone();
    for entity in transforms.keys() {
        newTransforms = newTransforms.update_with_key(&entity, |e, t| {
            applyMovement((Some(t), motions.get(&e)))
        });
    }
    newTransforms.clone()
}

fn applyMovement<'a, 'b>(input: (Option<Arc<&'a Transform>>, Option<Arc<&'a Motion>>)) -> Option<Arc<&'b Transform>> {
    match input {
        (Some(t), Some(m)) => {
            let t = Transform{
                x: t.x + m.accX,
                y: t.y + m.accY
            };
            Some(Arc::new(&t))
        },
        (Some(t), None) => Some(t),
        (None, _) => None
    }
}