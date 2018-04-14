use im::HashMap;
use entity::Entity;

pub struct ComponentManager<T> {
  entities: HashMap<Entity, T>
}

impl<T> ComponentManager<T> {
  pub fn new() -> Self {
    ComponentManager {
      entities: HashMap::new()
    }
  }
}
