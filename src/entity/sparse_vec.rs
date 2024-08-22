use crate::entity::{DenseEntity, Entity};
use alloc::vec::Vec;
use core::{fmt, iter, mem};

/// Maps entities to dense indexes.
#[derive(Clone, Default)]
pub struct SparseVec {
    entities: Vec<Option<DenseEntity>>,
}

impl SparseVec {
    /// Creates a new sparse vec.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Returns the dense entity mapped to `entity`, if any.
    #[inline]
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<DenseEntity> {
        self.entities
            .get(entity.sparse())?
            .filter(|e| e.version == entity.version)
    }

    /// Returns whether the sparse vec contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .is_some_and(|e| e.version == entity.version)
    }

    /// Removes `entity` from the sparse vec.
    ///
    /// Returns the removed dense entity, if any.
    #[inline]
    pub fn remove(&mut self, entity: Entity) -> Option<DenseEntity> {
        let entity_slot = self.entities.get_mut(entity.sparse())?;

        if entity_slot.is_some_and(|e| e.version == entity.version) {
            entity_slot.take()
        } else {
            None
        }
    }

    /// Returns the dense entity at the given sparse index, if any.
    #[inline]
    #[must_use]
    pub fn get_sparse(&self, sparse: usize) -> Option<DenseEntity> {
        *self.entities.get(sparse)?
    }

    /// Returns the dense index at the given sparse index without checking if it valid.
    #[inline]
    #[must_use]
    pub unsafe fn get_sparse_unchecked(&self, sparse: usize) -> usize {
        self.entities
            .get_unchecked(sparse)
            .unwrap_unchecked()
            .dense()
    }

    /// Returns whether the sparse vec contains the given sparse index.
    #[inline]
    #[must_use]
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.entities.get(sparse).and_then(Option::as_ref).is_some()
    }

    /// Removes the dense entity at the given sparse index.
    ///
    /// Returns the removed dense entity, if any.
    #[inline]
    pub fn remove_sparse(&mut self, sparse: usize) -> Option<DenseEntity> {
        self.entities.get_mut(sparse)?.take()
    }

    /// Returns the entity slot at the given dense index without checking if it is valid.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<DenseEntity> {
        self.entities.get_unchecked_mut(index)
    }

    /// Returns or allocates the entity slot at the given dense index.
    #[inline]
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<DenseEntity> {
        if index >= self.entities.len() {
            self.extend_to_index(index);
        }

        unsafe { self.entities.get_unchecked_mut(index) }
    }

    /// Swaps the entities at the given dense indexes without checking their validity.
    #[inline]
    pub unsafe fn swap(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.entities.len());
        debug_assert!(b < self.entities.len());
        debug_assert_ne!(a, b);

        let entity_a = &mut *self.entities.as_mut_ptr().add(a);
        let entity_b = &mut *self.entities.as_mut_ptr().add(b);
        mem::swap(entity_a, entity_b);
    }

    /// Removes all entities from the storage.
    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    #[cold]
    fn extend_to_index(&mut self, index: usize) {
        let extra_len =
            index.checked_next_power_of_two().unwrap_or(index) - self.entities.len() + 1;

        self.entities.extend(iter::repeat(None).take(extra_len));
    }
}

impl fmt::Debug for SparseVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries = self
            .entities
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| Some((i, e?)));

        f.debug_map().entries(entries).finish()
    }
}
