use crate::storage::{DenseEntity, Entity, SparseArray};
use std::convert::AsRef;

/// Sparse set-based storage for entities.
#[derive(Clone, Default, Debug)]
pub struct EntitySparseSet {
    sparse: SparseArray,
    entities: Vec<Entity>,
}

impl EntitySparseSet {
    /// Inserts `entity` into the storage.
    pub fn insert(&mut self, entity: Entity) -> Option<Entity> {
        let dense_entity = self.sparse.get_mut_or_allocate_at(entity.sparse());

        match dense_entity {
            Some(dense_entity) => unsafe {
                Some(std::mem::replace(
                    self.entities.get_unchecked_mut(dense_entity.dense()),
                    entity,
                ))
            },
            None => {
                *dense_entity = Some(DenseEntity::new(
                    self.entities.len() as u32,
                    entity.version(),
                ));

                self.entities.push(entity);
                None
            }
        }
    }

    /// Removes `entity` from the storage and returns `true` if it was successfully removed.
    pub fn remove(&mut self, entity: Entity) -> bool {
        let dense_index = match self.sparse.remove(entity) {
            Some(dense_entity) => dense_entity.dense(),
            None => return false,
        };

        self.entities.swap_remove(dense_index);

        if let Some(entity) = self.entities.get(dense_index) {
            let new_dense_entity = DenseEntity::new(dense_index as u32, entity.version());

            unsafe {
                *self.sparse.get_unchecked_mut(entity.sparse()) = Some(new_dense_entity);
            }
        }

        true
    }

    /// Returns `true` if the storage contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    /// Returns the number of entities in the storage.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns whether the storage contains no entities.
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
    #[inline]
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.entities.clear();
    }
}

impl AsRef<[Entity]> for EntitySparseSet {
    #[inline]
    fn as_ref(&self) -> &[Entity] {
        &self.entities
    }
}
