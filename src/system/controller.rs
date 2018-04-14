use control::Control;
use component::{
  ComponentManager,
  Joystick,
  Motion
};
use std::ops::Deref;
use im::HashSet;

pub struct Controller {
  player_speed: f32
}

impl Controller {
  pub fn new(player_speed: f32) -> Self {
    Controller{ player_speed }
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
            self.map_motion(acc, c.deref())
          })
        })
        .map(|m| {
          new_motions = new_motions.set(&entity, m);
        });
    }
    new_motions
  }

  fn map_motion(&self, orig: Motion, c: &Control) -> Motion {
    match c {
      Control::MoveUp => Motion{ velo_y: -self.player_speed, .. orig },
      Control::MoveDown => Motion{ velo_y: self.player_speed, .. orig },
      Control::MoveLeft => Motion{ velo_x: -self.player_speed, .. orig },
      Control::MoveRight => Motion{ velo_x: self.player_speed, .. orig },
    }
  }
}
