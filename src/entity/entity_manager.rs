use crate::entity::{Entity, EntityAllocator, EntitySparseSet};
use std::fmt;

pub struct EntityManager {
    allocator: EntityAllocator,
    entities: EntitySparseSet,
}

impl EntityManager {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            entities: EntitySparseSet::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn create(&mut self) -> Option<Entity> {
        let entity = self.allocator.allocate()?;
        self.entities.insert(entity);
        Some(entity)
    }

    #[inline]
    #[must_use]
    pub fn create_atomic(&self) -> Option<Entity> {
        self.allocator.allocate_atomic()
    }

    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.allocator.recycle(entity);
        true
    }

    pub fn clear(&mut self) {
        self.maintain();

        self.entities
            .as_slice()
            .iter()
            .for_each(|&entity| self.allocator.recycle(entity));

        self.entities.clear();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.allocator.reset();
        self.entities.clear();
    }

    #[inline]
    pub fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| {
            self.entities.insert(entity);
        });
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }
}

impl fmt::Debug for EntityManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.entities.as_slice()).finish()
    }
}
