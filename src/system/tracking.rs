use component::{ComponentManager, Follow, Motion, Transform};
use std::ops::Deref;

pub struct Tracking {}

impl Tracking {
  pub fn new() -> Self {
    Tracking{}
  }

  pub fn apply(
    &self,
    ticks: u32,
    followers: &ComponentManager<Follow>,
    transforms: &ComponentManager<Transform>,
    motions: ComponentManager<Motion>) -> ComponentManager<Motion> {
    let mut new_motions = motions;
    for entity in followers.keys() {
      followers.get(&entity).map(|follow| {
        match (transforms.get(&follow.target), transforms.get(&entity), new_motions.get(&entity)) {
          (Some(target_transform), Some(current_transform), Some(m)) => {
            let diff_x = target_transform.x - current_transform.x;
            let diff_y = target_transform.y - current_transform.y;

            let mov_x = if ticks > 0 { min(diff_x * follow.speed, diff_x * (1000.0 / ticks as f32)) } else { 0.0 };
            let mov_y = if ticks > 0 { min(diff_y * follow.speed, diff_y * (1000.0 / ticks as f32)) } else { 0.0 };
            println!("Tracking! x: {}; y: {}", mov_x, mov_y);
            new_motions = new_motions.set(&entity, Motion{
              velo_x: mov_x,
              velo_y: mov_y,
              .. *m.deref()
            });
          },
          (_, _, _) => {}
        }
      });
    }

    new_motions
  }
}

fn min(a: f32, b: f32) -> f32 {
  if a > b {
    b
  } else {
    a
  }
}
