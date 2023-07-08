use crate::storage::{Entity, IndexEntity};

/// Maps versioned sparse indexes (entities) to dense indexes.
/// Used internally by `ComponentStorage`.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct SparseArray {
    entities: Vec<Option<IndexEntity>>,
}

impl SparseArray {
    /// Returns the dense index mapped to `entity`, if any.
    pub fn get(&self, entity: Entity) -> Option<usize> {
        self.entities
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .filter(|index_entity| index_entity.version() == entity.version())
            .map(IndexEntity::dense)
    }

    /// Returns `true` if the array contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .filter(|index_entity| index_entity.version() == entity.version())
            .is_some()
    }

    /// Returns the dense index mapped to `sparse`, if any.
    pub fn get_from_sparse(&self, sparse: usize) -> Option<usize> {
        self.entities
            .get(sparse)
            .and_then(Option::as_ref)
            .map(IndexEntity::dense)
    }

    /// Returns the dense index mapped to `sparse`, without checking if the index is valid.
    pub unsafe fn get_from_sparse_unchecked(&self, sparse: usize) -> usize {
        self.entities
            .get_unchecked(sparse)
            .unwrap_unchecked()
            .dense()
    }

    /// Returns `true` if the array contains `sparse`.
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.entities.get(sparse).and_then(Option::as_ref).is_some()
    }

    /// Removes `entity` from the array and returns the dense index mapped to it, if any.
    pub(crate) fn remove(&mut self, entity: Entity) -> Option<usize> {
        self.entities
            .get_mut(entity.sparse())?
            .take()
            .map(|index_entity| index_entity.dense())
    }

    /// Returns the `IndexEntity` slot at `index` without checking if the `index` is valid.
    pub(crate) unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
        self.entities.get_unchecked_mut(index)
    }

    /// Returns the `IndexEntity` slot at `index`. Will allocate memory if the index cannot be
    /// stored in the allocated pages.
    pub(crate) fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<IndexEntity> {
        if index >= self.entities.len() {
            let extra_len =
                index.checked_next_power_of_two().unwrap_or(index) - self.entities.len() + 1;

            self.entities
                .extend(std::iter::repeat(None).take(extra_len));
        }

        &mut self.entities[index]
    }

    /// Swaps the indexes at `a` and `b` without checking if `a` and `b` are valid sparse indexes.
    pub(crate) unsafe fn swap_nonoverlapping_unchecked(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.entities.len());
        debug_assert!(b < self.entities.len());
        debug_assert_ne!(a, b);

        let entity_a = &mut *self.entities.as_mut_ptr().add(a);
        let entity_b = &mut *self.entities.as_mut_ptr().add(b);
        std::mem::swap(entity_a, entity_b);
    }

    /// Removes all entities from the array.
    pub(crate) fn clear(&mut self) {
        self.entities.clear();
    }
}
