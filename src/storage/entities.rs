use crate::registry::{BorrowWorld, World};
use crate::storage::{Entity, SparseSet};
use std::ops::Deref;

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
                Entity::with_id(index as u32)
            }
        }
    }

    fn remove(&mut self, entity: Entity) {
        self.removed.push(entity)
    }
}

#[derive(Default)]
pub struct EntityStorage {
    allocator: EntityAllocator,
    entities: SparseSet<()>,
}

impl EntityStorage {
    pub fn create(&mut self) -> Entity {
        let entity = self.allocator.allocate();
        self.entities.insert(entity, ());
        entity
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if self.entities.remove(entity).is_some() {
            self.allocator.remove(entity);
            true
        } else {
            false
        }
    }
}

pub struct Entities<'a> {
    storage: &'a EntityStorage,
}

impl Deref for Entities<'_> {
    type Target = EntityStorage;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a> BorrowWorld<'a> for Entities<'a> {
    fn borrow_world(world: &'a World) -> Self {
        Self {
            storage: world.entities(),
        }
    }
}
