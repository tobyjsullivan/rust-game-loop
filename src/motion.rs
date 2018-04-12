use im::shared::Shared;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Motion {
    pub velo_x: i32,
    pub velo_y: i32,
    pub next_move: i32,
    pub move_rate: i32
}
