use crate::storage::{DenseEntity, Entity};

/// Maps versioned sparse indexes (entities) to dense indexes.
/// Used internally by `ComponentStorage`.
#[derive(Clone, Debug, Default)]
pub struct SparseArray {
    entities: Vec<Option<DenseEntity>>,
}

impl SparseArray {
    /// Returns the index entity mapped to `entity`, if any.
    #[inline]
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<DenseEntity> {
        self.entities
            .get(entity.sparse())?
            .filter(|dense_entity| dense_entity.version() == entity.version())
    }

    /// Returns `true` if the array contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .is_some_and(|dense_entity| dense_entity.version() == entity.version())
    }

    /// Removes the entity from the array.
    #[inline]
    pub fn remove(&mut self, entity: Entity) -> Option<DenseEntity> {
        let entity_slot = self.entities.get_mut(entity.sparse())?;

        if entity_slot.is_some_and(|dense_entity| dense_entity.version() == entity.version()) {
            entity_slot.take()
        } else {
            None
        }
    }

    /// Returns the index entity mapped to `sparse`, if any.
    #[inline]
    #[must_use]
    pub fn get_sparse(&self, sparse: usize) -> Option<DenseEntity> {
        *self.entities.get(sparse)?
    }

    /// Returns the dense index mapped to `sparse`, without checking if the index is valid.
    #[inline]
    #[must_use]
    pub unsafe fn get_sparse_unchecked(&self, sparse: usize) -> usize {
        self.entities
            .get_unchecked(sparse)
            .unwrap_unchecked()
            .dense()
    }

    /// Returns `true` if the array contains `sparse`.
    #[inline]
    #[must_use]
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.entities.get(sparse).and_then(Option::as_ref).is_some()
    }

    /// Removes the entity at the `sparse` index from the array.
    #[inline]
    pub fn remove_sparse(&mut self, sparse: usize) -> Option<DenseEntity> {
        self.entities.get_mut(sparse)?.take()
    }

    /// Returns the `IndexEntity` slot at `index` without checking if the `index` is valid.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<DenseEntity> {
        self.entities.get_unchecked_mut(index)
    }

    /// Returns the `IndexEntity` slot at `index`. Will allocate memory if the index cannot be
    /// stored in the allocated pages.
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<DenseEntity> {
        if index >= self.entities.len() {
            let extra_len =
                index.checked_next_power_of_two().unwrap_or(index) - self.entities.len() + 1;

            self.entities
                .extend(std::iter::repeat(None).take(extra_len));
        }

        &mut self.entities[index]
    }

    /// Swaps the entities at the sparse indexes `a` and `b` without checking if they are valid.
    #[inline]
    pub unsafe fn swap_nonoverlapping_unchecked(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.entities.len());
        debug_assert!(b < self.entities.len());
        debug_assert_ne!(a, b);

        let entity_a = &mut *self.entities.as_mut_ptr().add(a);
        let entity_b = &mut *self.entities.as_mut_ptr().add(b);
        std::mem::swap(entity_a, entity_b);
    }

    /// Removes all entities from the array.
    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
    }
}
