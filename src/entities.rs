use crate::{Entity, SparseArray, SparseSet};

#[derive(Default, Debug)]
struct EntityAllocator {
    index: usize,
    removed: Vec<Entity>,
}

impl EntityAllocator {
    fn allocate(&mut self) -> Entity {
        match self.removed.pop() {
            Some(entity) => Entity::new(entity.id(), entity.gen() + 1),
            None => {
                let index = self.index;
                self.index += 1;
                Entity::new(index as u32, 0)
            }
        }
    }

    fn remove(&mut self, entity: Entity) {
        self.removed.push(entity)
    }
}

#[derive(Default)]
pub struct Entities {
    allocator: EntityAllocator,
    entities: SparseSet<()>,
}

impl Entities {
    pub fn create(&mut self) -> Entity {
        let entity = self.allocator.allocate();
        self.entities.insert(entity, ());
        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        if self.entities.remove(entity).is_some() {
            self.allocator.remove(entity)
        }
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[()]) {
        self.entities.split()
    }
}
