use crate::entity::{DenseEntity, Entity};
use std::{fmt, iter, mem};

#[derive(Clone, Default)]
pub struct SparseVec {
    entities: Vec<Option<DenseEntity>>,
}

impl SparseVec {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<DenseEntity> {
        self.entities
            .get(entity.sparse())?
            .filter(|e| e.version == entity.version)
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .is_some_and(|e| e.version == entity.version)
    }

    #[inline]
    pub fn remove(&mut self, entity: Entity) -> Option<DenseEntity> {
        let entity_slot = self.entities.get_mut(entity.sparse())?;

        if entity_slot.is_some_and(|e| e.version == entity.version) {
            entity_slot.take()
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn get_sparse(&self, sparse: usize) -> Option<DenseEntity> {
        *self.entities.get(sparse)?
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_sparse_unchecked(&self, sparse: usize) -> usize {
        self.entities
            .get_unchecked(sparse)
            .unwrap_unchecked()
            .dense()
    }

    #[inline]
    #[must_use]
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.entities.get(sparse).and_then(Option::as_ref).is_some()
    }

    #[inline]
    pub fn remove_sparse(&mut self, sparse: usize) -> Option<DenseEntity> {
        self.entities.get_mut(sparse)?.take()
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<DenseEntity> {
        self.entities.get_unchecked_mut(index)
    }

    #[inline]
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<DenseEntity> {
        if index >= self.entities.len() {
            let extra_len =
                index.checked_next_power_of_two().unwrap_or(index) - self.entities.len() + 1;

            self.entities.extend(iter::repeat(None).take(extra_len));
        }

        unsafe { self.entities.get_unchecked_mut(index) }
    }

    #[inline]
    pub unsafe fn swap(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.entities.len());
        debug_assert!(b < self.entities.len());
        debug_assert_ne!(a, b);

        let entity_a = &mut *self.entities.as_mut_ptr().add(a);
        let entity_b = &mut *self.entities.as_mut_ptr().add(b);
        mem::swap(entity_a, entity_b);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
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
