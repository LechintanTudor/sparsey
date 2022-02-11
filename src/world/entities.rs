use crate::storage::{Entity, EntityStorage};

/// View over all entities in a world. Allows creating new entities atomically.
#[derive(Clone, Copy)]
pub struct Entities<'a> {
    storage: &'a EntityStorage,
}

impl<'a> Entities<'a> {
    pub(crate) fn new(storage: &'a EntityStorage) -> Self {
        Self { storage }
    }

    /// Atomically creates a new entity.
    #[inline]
    pub fn create(&self) -> Entity {
        self.storage.create_atomic()
    }

    /// Returns `true` if the view contains `entity`.
    #[inline]
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of entities in the view.
    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view contains no entities.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all entities in the view as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[Entity] {
        self.storage
    }
}
