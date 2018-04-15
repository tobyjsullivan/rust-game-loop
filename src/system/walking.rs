use std::ops::Deref;

use component::{
  ComponentManager,
  Walk,
  Motion,
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
    motions: ComponentManager<Motion>,
    transforms: &ComponentManager<Transform>
    ) -> (ComponentManager<Walk>, ComponentManager<Motion>) {

    let mut new_motions = motions;
    let mut new_walkers = walkers;
    for entity in new_walkers.keys() {
      let (motion, walk) = match (new_walkers.get(&entity), transforms.get(&entity)) {
        (Some(w), Some(t)) => {
          if w.in_motion() {
            (Some(generate_motion(&w, &t)), Some(Walk{step: w.step + ticks as i32, .. *w.deref()}))
          } else {
            (None, None)
          }
        },
        _ => (None, None)
      };

      new_motions = match motion {
        Some(m) => new_motions.set(&entity, m),
        None => new_motions.remove(&entity)
      };

      new_walkers = match walk {
        Some(w) => new_walkers.set(&entity, w),
        None => new_walkers.remove(&entity)
      };
    }

    (new_walkers, new_motions)
  }
}

fn generate_motion(w: &Walk, t: &Transform) -> Motion {
  let rem_steps = w.speed - w.step;
  let dist_x = w.dest_x - t.x;
  let dist_y = w.dest_y - t.y;

  let velo_x = dist_x / (rem_steps as f32 / 1000.0);
  let velo_y = dist_y / (rem_steps as f32 / 1000.0);
  Motion{velo_x, velo_y}
}