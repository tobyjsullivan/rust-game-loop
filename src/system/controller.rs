use control::Control;
use component::{
  ComponentManager,
  Joystick,
  Transform,
  Walk
};
use std::ops::Deref;
use im::HashSet;

pub struct Controller {
  player_speed: f64
}

impl Controller {
  pub fn new(player_speed: f64) -> Self {
    Controller{ player_speed }
  }

  pub fn apply(
    &self,
    _ticks: u32,
    controls: &HashSet<Control>,
    joysticks: &ComponentManager<Joystick>,
    walkers: ComponentManager<Walk>,
    transforms: &ComponentManager<Transform>
    ) -> ComponentManager<Walk> {

    let mut new_walkers = walkers;
    for entity in joysticks.keys() {
      let odir = controls.iter().filter(|c| is_directional_control(c)).next();
      let new_walk = match (odir, new_walkers.get(&entity), transforms.get(&entity)) {
        (Some(dir), Some(w), Some(t)) => {
          // Check if ready for a new walk
          if w.in_motion() {
            None
          } else {
            Some(generate_walk(&dir, self.player_speed, &t))
          }
        },
        (Some(dir), None, Some(t)) => {
          // Add a new walk
          Some(generate_walk(&dir, self.player_speed, &t))
        },
        _ => {
          None
        }
      };

      new_walkers = match new_walk {
        Some(w) => new_walkers.set(&entity, w),
        None => new_walkers
      };
    }
    new_walkers
  }
}

fn is_directional_control(c: &Control) -> bool {
  match c {
    Control::MoveUp => true,
    Control::MoveDown => true,
    Control::MoveLeft => true,
    Control::MoveRight => true
  }
}

fn generate_walk(c: &Control, player_speed: f64, t: &Transform) -> Walk {
  let (dest_x, dest_y) = match c {
    Control::MoveUp => (t.x, t.y - 1.0),
    Control::MoveDown => (t.x, t.y + 1.0),
    Control::MoveLeft => (t.x - 1.0, t.y),
    Control::MoveRight => (t.x + 1.0, t.y)
  };
  Walk{transition_time: (1000.0 / player_speed) as i32, step: 0, dest_x: dest_x.round(), dest_y: dest_y.round()}
}
