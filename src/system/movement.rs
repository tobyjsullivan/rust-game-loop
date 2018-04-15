use std::ops::Deref;

use component::{
  ComponentManager,
  TransitionalMotion,
  Transform
};

pub struct Movement {}

impl Movement {
  pub fn new() -> Self {
    Movement{}
  }

  pub fn apply(
    &self,
    ticks: u32,
    motions: ComponentManager<TransitionalMotion>,
    transforms: ComponentManager<Transform>
    ) -> (ComponentManager<TransitionalMotion>, ComponentManager<Transform>) {

    let mut new_transforms = transforms;
    let mut new_motions = motions;
    for entity in new_motions.keys() {
      let (transform, motion) = match (new_motions.get(&entity), new_transforms.get(&entity)) {
        (Some(m), Some(t)) => {
          if m.in_motion() {
            (Some(generate_transform(ticks, &m, &t)), Some(TransitionalMotion{step: m.step + ticks as i32, .. *m.deref()}))
          } else {
            (Some(Transform{
              x: t.x.round(),
              y: t.y.round(),
              .. *t.deref()
            }), None)
          }
        },
        _ => (None, None)
      };

      new_transforms = match transform {
        Some(t) => new_transforms.set(&entity, t),
        None => new_transforms
      };

      new_motions = match motion {
        Some(m) => new_motions.set(&entity, m),
        None => new_motions.remove(&entity)
      };
    }

    (new_motions, new_transforms)
  }
}

fn generate_transform(ticks: u32, m: &TransitionalMotion, t: &Transform) -> Transform {
  let rem_dist_x = m.dest_x - t.x;
  let rem_dist_y = m.dest_y - t.y;
  let rem_steps = m.transition_time - m.step;
  let dist_x_per_step = rem_dist_x / rem_steps as f64;
  let dist_y_per_step = rem_dist_y / rem_steps as f64;
  let current_travel_x = dist_x_per_step * ticks as f64;
  let current_travel_y = dist_y_per_step * ticks as f64;
  Transform {
    x: t.x + current_travel_x,
    y: t.y + current_travel_y,
    .. *t
  }
}