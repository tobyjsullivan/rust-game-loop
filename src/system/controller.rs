use control::Control;
use component::{
  ComponentManager,
  Joystick,
  Transform,
  TransitionalMotion,
  Collidable
};
use entity::Entity;
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
    motions: ComponentManager<TransitionalMotion>,
    transforms: &ComponentManager<Transform>,
    collidables: &ComponentManager<Collidable>
    ) -> ComponentManager<TransitionalMotion> {

    let mut new_motions = motions;
    for entity in joysticks.keys() {
      let odir = controls.iter().filter(|c| is_directional_control(c)).next();
      let motion = match (odir, new_motions.get(&entity), transforms.get(&entity)) {
        (Some(dir), Some(m), Some(t)) => {
          // Check if ready for a new motion
          if m.in_motion() {
            None
          } else {
            // Check if move would collide with something
            generate_motion(&dir, self.player_speed, &t, collidables, transforms)
          }
        },
        (Some(dir), None, Some(t)) => {
          // Add a new motion
          generate_motion(&dir, self.player_speed, &t, collidables, transforms)
        },
        _ => {
          None
        }
      };

      new_motions = match motion {
        Some(m) => new_motions.set(&entity, m),
        None => new_motions
      };
    }
    new_motions
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

fn generate_motion(c: &Control, player_speed: f64, t: &Transform, collidables: &ComponentManager<Collidable>, transforms: &ComponentManager<Transform>) -> Option<TransitionalMotion> {
  let (dest_x, dest_y) = match c {
    Control::MoveUp => (t.x, t.y - 1.0),
    Control::MoveDown => (t.x, t.y + 1.0),
    Control::MoveLeft => (t.x - 1.0, t.y),
    Control::MoveRight => (t.x + 1.0, t.y)
  };
  if !find_colliding_entities(dest_x.round() as i32, dest_y.round() as i32, collidables, transforms).is_empty() {
    None
  } else {
    Some(TransitionalMotion{transition_time: (1000.0 / player_speed) as i32, step: 0, dest_x: dest_x.round(), dest_y: dest_y.round()})
  }
}

fn find_colliding_entities(pos_x: i32, pos_y: i32, collidables: &ComponentManager<Collidable>, transforms: &ComponentManager<Transform>) -> HashSet<Entity> {
  let mut out: HashSet<Entity> = HashSet::new();
  for entity in collidables.keys() {
    match (collidables.get(&entity), transforms.get(&entity)) {
      (Some(c), Some(t)) => {
        if c.obstruction && t.x.round() as i32 == pos_x && t.y.round() as i32 == pos_y {
          out = out.insert(entity);
        }
      },
      _ => {}
    }
  }

  out
}
