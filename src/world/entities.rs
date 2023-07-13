use crate::storage::{Entity, EntityStorage};
use std::fmt;

/// View over all entities in a world. Allows creating new entities atomically.
#[derive(Clone, Copy)]
pub struct Entities<'a> {
    storage: &'a EntityStorage,
}

impl<'a> Entities<'a> {
    pub(crate) fn new(storage: &'a EntityStorage) -> Self {
        Self { storage }
    }

    /// Creates a new entity atomically. The entity isn't saved to the main storage until
    /// `World::maintain` is called.
    #[inline]
    pub fn create(&self) -> Entity {
        self.storage.create_atomic()
    }

    /// Returns `true` if the view contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of entities in the view.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view contains no entities.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all entities in the view as a slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Entity] {
        self.storage.as_slice()
    }
}

impl fmt::Debug for Entities<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
