pub struct Walk {
  pub transition_time: i32, // how many ticks it should take to move 1 unit
  pub step: i32, // how many ticks have occured in this move
  pub dest_x: f64,
  pub dest_y: f64
}

impl Walk {
  pub fn in_motion(&self) -> bool {
    self.step < self.transition_time
  }
}
