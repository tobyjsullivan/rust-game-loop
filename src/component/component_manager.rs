use im::HashMap;
use entity::Entity;
use std::sync::Arc;
use std::iter::FromIterator;

pub struct ComponentManager<T> {
  entities: HashMap<Entity, T>
}

impl<T> ComponentManager<T> {
  pub fn new() -> Self {
    ComponentManager {
      entities: HashMap::new()
    }
  }

  pub fn set(&self, e: &Entity, comp: T) -> Self {
    ComponentManager {
      entities: self.entities.set(e.clone(), comp),
      .. *self
    }
  }

  pub fn get(&self, e: &Entity) -> Option<Arc<T>> {
      self.entities.get(e)
  }

  pub fn keys(&self) -> KeyIterator {
    KeyIterator{
      entities: Vec::from_iter(self.entities.keys())
    }
  }

  pub fn remove(&self, e: &Entity) -> Self {
    ComponentManager {
      entities: self.entities.remove(e),
      .. *self
    }
  }
}

pub struct KeyIterator {
  entities: Vec<Arc<Entity>>
}

impl Iterator for KeyIterator {
  type Item = Arc<Entity>;

  fn next(&mut self) -> Option<Arc<Entity>> {
    self.entities.pop()
  }
}
