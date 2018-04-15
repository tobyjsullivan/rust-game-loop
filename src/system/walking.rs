use std::ops::Deref;

use component::{
  ComponentManager,
  Walk,
  Transform
};

pub struct Walking {}

impl Walking {
  pub fn new() -> Self {
    Walking{}
  }

  pub fn apply(
    &self,
    ticks: u32,
    walkers: ComponentManager<Walk>,
    transforms: ComponentManager<Transform>
    ) -> (ComponentManager<Walk>, ComponentManager<Transform>) {

    let mut new_transforms = transforms;
    let mut new_walkers = walkers;
    for entity in new_walkers.keys() {
      let (transform, walk) = match (new_walkers.get(&entity), new_transforms.get(&entity)) {
        (Some(w), Some(t)) => {
          if w.in_motion() {
            (Some(generate_transform(ticks, &w, &t)), Some(Walk{step: w.step + ticks as i32, .. *w.deref()}))
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

      new_walkers = match walk {
        Some(w) => new_walkers.set(&entity, w),
        None => new_walkers.remove(&entity)
      };
    }

    (new_walkers, new_transforms)
  }
}

fn generate_transform(ticks: u32, w: &Walk, t: &Transform) -> Transform {
  let rem_dist_x = w.dest_x - t.x;
  let rem_dist_y = w.dest_y - t.y;
  let rem_steps = w.transition_time - w.step;
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