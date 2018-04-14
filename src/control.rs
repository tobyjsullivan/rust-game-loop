use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use im::HashSet;

#[derive(Hash, Eq, PartialEq)]
pub enum Control {
  MoveDown,
  MoveUp,
  MoveLeft,
  MoveRight
}

enum ControlChange {
  Begin(Control),
  End(Control),
  None
}

impl Control {
  pub fn mutate_controls(controls: HashSet<Control>, event: Event) -> HashSet<Control> {
    match from_event(event) {
      ControlChange::Begin(c) => {
        controls.insert(c)
      },
      ControlChange::End(c) => {
        controls.remove(&c)
      },
      ControlChange::None => controls
    }
  }
}

fn from_event(event: Event) -> ControlChange {
  match event {
    Event::KeyDown { keycode: Some(Keycode::W), .. } => ControlChange::Begin(Control::MoveUp),
    Event::KeyDown { keycode: Some(Keycode::A), .. } => ControlChange::Begin(Control::MoveLeft),
    Event::KeyDown { keycode: Some(Keycode::S), .. } => ControlChange::Begin(Control::MoveDown),
    Event::KeyDown { keycode: Some(Keycode::D), .. } => ControlChange::Begin(Control::MoveRight),
    Event::KeyUp { keycode: Some(Keycode::W), .. } => ControlChange::End(Control::MoveUp),
    Event::KeyUp { keycode: Some(Keycode::A), .. } => ControlChange::End(Control::MoveLeft),
    Event::KeyUp { keycode: Some(Keycode::S), .. } => ControlChange::End(Control::MoveDown),
    Event::KeyUp { keycode: Some(Keycode::D), .. } => ControlChange::End(Control::MoveRight),
    _ => ControlChange::None
  }
}
