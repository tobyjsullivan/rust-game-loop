use control::Control;
use component::{
  ComponentManager,
  Joystick,
  Motion
};
use std::ops::Deref;
use im::HashSet;

pub struct Controller {}

impl Controller {
  pub fn new() -> Self {
    Controller{}
  }

  pub fn apply(
    &self,
    _ticks: u32,
    controls: &HashSet<Control>,
    joysticks: &ComponentManager<Joystick>,
    motions: ComponentManager<Motion>
    ) -> ComponentManager<Motion> {

    let mut new_motions = motions;
    for entity in joysticks.keys() {
      new_motions.get(&entity)
        .map(|m| {
          controls.iter().fold(Motion{ velo_x: 0.0, velo_y: 0.0, .. *m.deref() }, |acc, c| {
            map_motion(acc, c.deref())
          })
        })
        .map(|m| {
          new_motions = new_motions.set(&entity, m);
        });
    }
    new_motions
  }
}

fn map_motion(orig: Motion, c: &Control) -> Motion {
  match c {
    Control::MoveUp => Motion{ velo_y: -1.0, .. orig },
    Control::MoveDown => Motion{ velo_y: 1.0, .. orig },
    Control::MoveLeft => Motion{ velo_x: -1.0, .. orig },
    Control::MoveRight => Motion{ velo_x: 1.0, .. orig },
    _ => orig
  }
}