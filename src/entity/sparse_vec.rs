use crate::entity::{Entity, Version};
use alloc::vec::Vec;
use core::{fmt, iter, mem};

/// Maps entities to dense indexes.
#[derive(Clone, Default)]
pub struct SparseVec {
    slots: Vec<Option<SparseVecSlot>>,
}

impl SparseVec {
    /// Creates a new sparse vec.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// Returns the dense index mapped to `entity`, if any.
    #[inline]
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<u32> {
        self.slots
            .get(entity.sparse())?
            .filter(|slot| slot.version == entity.version)
            .map(|slot| slot.index)
    }

    /// Returns whether the sparse vec contains `entity`.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.slots
            .get(entity.sparse())
            .and_then(Option::as_ref)
            .is_some_and(|slot| slot.version == entity.version)
    }

    /// Removes `entity` from the sparse vec.
    ///
    /// Returns the dense index mapped to `entity`, if any.
    #[inline]
    pub fn remove(&mut self, entity: Entity) -> Option<u32> {
        self.slots
            .get_mut(entity.sparse())?
            .take_if(|slot| slot.version == entity.version)
            .map(|slot| slot.index)
    }

    /// Returns the dense index at the given sparse index, if any.
    #[inline]
    #[must_use]
    pub fn get_sparse(&self, sparse: usize) -> Option<u32> {
        self.slots.get(sparse)?.map(|slot| slot.index)
    }

    /// Returns the dense index at the given sparse index without checking if it
    /// valid.
    #[inline]
    #[must_use]
    pub unsafe fn get_sparse_unchecked(&self, sparse: usize) -> usize {
        self.slots.get_unchecked(sparse).unwrap_unchecked().dense()
    }

    /// Returns whether the sparse vec contains the given sparse index.
    #[inline]
    #[must_use]
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.slots.get(sparse).and_then(Option::as_ref).is_some()
    }

    /// Removes the dense entity at the given sparse index.
    ///
    /// Returns the removed dense entity, if any.
    #[inline]
    pub fn remove_sparse(&mut self, sparse: usize) -> Option<SparseVecSlot> {
        self.slots.get_mut(sparse)?.take()
    }

    /// Returns the entity slot at the given dense index without checking if it
    /// is valid.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<SparseVecSlot> {
        self.slots.get_unchecked_mut(index)
    }

    /// Returns or allocates the entity slot at the given dense index.
    #[inline]
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<SparseVecSlot> {
        if index >= self.slots.len() {
            self.extend_to_index(index);
        }

        unsafe { self.slots.get_unchecked_mut(index) }
    }

    /// Swaps the entities at the given dense indexes without checking their
    /// validity.
    #[inline]
    pub unsafe fn swap_nonoverlapping(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.slots.len());
        debug_assert!(b < self.slots.len());
        debug_assert_ne!(a, b);

        let index_a = &mut (*self.slots.as_mut_ptr().add(a))
            .as_mut()
            .unwrap_unchecked()
            .index;

        let index_b = &mut (*self.slots.as_mut_ptr().add(b))
            .as_mut()
            .unwrap_unchecked()
            .index;

        mem::swap(index_a, index_b);
    }

    /// Removes all entities from the storage.
    #[inline]
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    #[cold]
    fn extend_to_index(&mut self, index: usize) {
        let extra_len = index.checked_next_power_of_two().unwrap_or(index) - self.slots.len() + 1;
        self.slots.extend(iter::repeat_n(None, extra_len));
    }
}

impl fmt::Debug for SparseVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, &dense_entity)| {
                let dense_entity = dense_entity?;
                let entity = Entity::new(i as u32, dense_entity.version);
                Some((entity, dense_entity.index))
            });

        f.debug_map().entries(entries).finish()
    }
}

/// Versioned dense index.
#[derive(Clone, Copy)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub struct SparseVecSlot {
    /// The index of the slot.
    pub index: u32,

    /// The version of the slot.
    pub version: Version,
}

impl SparseVecSlot {
    /// Returns the `index` extended to a [`usize`].
    #[inline]
    #[must_use]
    pub fn dense(&self) -> usize {
        self.index as usize
    }
}

impl fmt::Debug for SparseVecSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(SparseVecSlot))
            .field("index", &self.index)
            .field("version", &self.version.0)
            .finish()
    }
}
