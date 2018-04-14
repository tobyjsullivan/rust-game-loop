
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Entity {
    pub id: i32
}

pub struct EntityProducer {
    counter: i32
}

impl EntityProducer {
    pub fn new() -> Self {
        EntityProducer{counter: 0}
    }

    pub fn create(&mut self) -> Entity {
        let id = self.counter;
        self.counter += 1;
        Entity { id }
    }
}