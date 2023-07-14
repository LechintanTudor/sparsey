use crate::storage::{Entity, EntityAllocator, EntitySparseSet};

/// Sparse set-based storage for entities.
#[derive(Default, Debug)]
pub struct EntityStorage {
    entities: EntitySparseSet,
    allocator: EntityAllocator,
}

impl EntityStorage {
    /// Creates a new `Entity` and returns it.
    #[inline]
    pub fn create(&mut self) -> Entity {
        let entity = self
            .allocator
            .allocate()
            .expect("No entities left to allocate");

        self.entities.insert(entity);
        entity
    }

    /// Atomically creates a new `Entity` and returns it.
    #[inline]
    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("No entities left to allocate")
    }

    /// Removes `entity` from the storage if it exits. Returns whether there was anything to remove.
    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.allocator.recycle(entity);
        true
    }

    /// Returns whether the storage contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns the number of entities in the storage.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns whether the storage is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns all entities in the storage as a slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    /// Removes all entities from the storage.
    pub fn clear(&mut self) {
        // Add the entities created atomically to the storage so they can be properly recycled
        self.maintain();

        self.entities
            .as_slice()
            .iter()
            .for_each(|&entity| self.allocator.recycle(entity));

        self.entities.clear();
    }

    /// Removes all entities from the storage and resets the entity allocator allowing previously
    /// generated entities to be generated again.
    #[inline]
    pub fn reset(&mut self) {
        self.entities.clear();
        self.allocator.reset();
    }

    /// Adds the entities created atomically to the storage.
    #[inline]
    pub fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| {
            let _ = self.entities.insert(entity);
        });
    }

    /// Returns the managed entity storage.
    #[inline]
    #[must_use]
    pub fn storage(&self) -> &EntitySparseSet {
        &self.entities
    }

    /// Returns the allocator used to create and recycle entities.
    #[inline]
    #[must_use]
    pub fn allocator(&self) -> &EntityAllocator {
        &self.allocator
    }

    /// Decomposes the entity storage into its underlying parts.
    #[inline]
    pub fn into_raw_parts(self) -> (EntitySparseSet, EntityAllocator) {
        (self.entities, self.allocator)
    }
}

impl AsRef<[Entity]> for EntityStorage {
    #[inline]
    fn as_ref(&self) -> &[Entity] {
        self.entities.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut entities = EntityStorage::default();

        // e0 created synchronously
        let e0 = entities.create();
        assert!(entities.contains(e0));

        // e1 created atomically, unmaintained
        let e1 = entities.create_atomic();
        assert!(!entities.contains(e1));

        // e1 created atomically, maintained
        entities.maintain();
        assert!(entities.contains(e1));
    }

    #[test]
    fn destroy() {
        let mut entities = EntityStorage::default();

        // e0 created synchronously
        let e0 = entities.create();
        assert!(entities.destroy(e0));
        assert!(!entities.destroy(e0));
        assert!(!entities.contains(e0));

        // e1 created atomically, unmaintained
        let e1 = entities.create_atomic();
        assert!(!entities.destroy(e1));
        assert!(!entities.contains(e1));

        // e1 created atomically maintained
        entities.maintain();
        assert!(entities.destroy(e1));
        assert!(!entities.destroy(e1));
        assert!(!entities.contains(e1));
    }
}
