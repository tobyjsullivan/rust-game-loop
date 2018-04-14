use entity::Entity;

pub struct Follow<'a> {
  pub target: &'a Entity,
  pub speed: f32
}