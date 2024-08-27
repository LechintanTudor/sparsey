use crate::entity::{Entity, EntityAllocator, EntitySparseSet};

#[derive(Default, Debug)]
pub(crate) struct EntityStorage {
    allocator: EntityAllocator,
    entities: EntitySparseSet,
}

impl EntityStorage {
    #[must_use]
    pub fn create(&mut self) -> Entity {
        let entity = self
            .allocator
            .allocate()
            .expect("No entities left to allocate");

        self.entities.insert(entity);
        entity
    }

    #[must_use]
    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("No entities left to allocate")
    }

    pub fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| {
            self.entities.insert(entity);
        });
    }

    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.allocator.recycle(entity);
        true
    }

    pub fn clear(&mut self) {
        let _ = self.allocator.maintain();
        self.entities.clear();
    }

    pub fn reset(&mut self) {
        self.allocator.reset();
        self.entities.clear();
    }
}
