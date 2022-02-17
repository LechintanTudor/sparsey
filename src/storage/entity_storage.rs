use crate::storage::{Entity, IndexEntity, SparseArray, Version};
use std::num::NonZeroU32;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Sparse set-based storage for entities.
#[doc(hidden)]
#[derive(Default)]
pub struct EntityStorage {
    storage: EntitySparseSet,
    allocator: EntityAllocator,
}

impl EntityStorage {
    /// Creates a new `Entity` and returns it.
    pub(crate) fn create(&mut self) -> Entity {
        let entity = self.allocator.allocate().expect("No entities left to allocate");
        self.storage.insert(entity);
        entity
    }

    /// Atomically creates a new `Entity` and returns it.
    pub(crate) fn create_atomic(&self) -> Entity {
        self.allocator.allocate_atomic().expect("No entities left to allocate")
    }

    /// Removes `entity` from the storage if it exits. Returns whether or not
    /// there was anything to remove.
    pub(crate) fn destroy(&mut self, entity: Entity) -> bool {
        self.maintain();

        if self.storage.remove(entity) {
            self.allocator.deallocate(entity);
            true
        } else {
            false
        }
    }

    /// Returns `true` if the storage contains `entity`.
    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Removes all entities from the storage.
    pub(crate) fn clear(&mut self) {
        self.storage.entities.iter().for_each(|&entity| self.allocator.deallocate(entity));
        self.storage.clear();
    }

    /// Adds the entities created atomically to the storage.
    pub(crate) fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| self.storage.insert(entity));
    }
}

impl Deref for EntityStorage {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.storage.entities
    }
}

#[derive(Clone, Default, Debug)]
struct EntitySparseSet {
    sparse: SparseArray,
    entities: Vec<Entity>,
}

impl EntitySparseSet {
    /// Inserts `entity` into the storage.
    fn insert(&mut self, entity: Entity) {
        let index_entity = self.sparse.get_mut_or_allocate_at(entity.sparse());

        match index_entity {
            Some(index_entity) => unsafe {
                *self.entities.get_unchecked_mut(index_entity.dense()) = entity;
            },
            None => {
                *index_entity =
                    Some(IndexEntity::new(self.entities.len() as u32, entity.version()));
                self.entities.push(entity);
            }
        }
    }

    /// Removes `entity` from the storage and returns `true` if it was successfully removed.
    fn remove(&mut self, entity: Entity) -> bool {
        let dense_index = match self.sparse.remove(entity) {
            Some(index) => index,
            None => return false,
        };

        self.entities.swap_remove(dense_index);

        if let Some(entity) = self.entities.get(dense_index) {
            let new_index_entity = IndexEntity::new(dense_index as u32, entity.version());

            unsafe {
                *self.sparse.get_unchecked_mut(entity.sparse()) = Some(new_index_entity);
            }
        }

        true
    }

    /// Returns `true` if the storage contains `entity`.
    fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    /// Removes all entities from the storage.
    fn clear(&mut self) {
        self.sparse.clear();
        self.entities.clear();
    }
}

#[derive(Default, Debug)]
struct EntityAllocator {
    next_id_to_allocate: AtomicU32,
    last_maintained_id: u32,
    recycled: Vec<Entity>,
    recycled_len: AtomicUsize,
}

impl EntityAllocator {
    /// Tries to allocate an `Entity` synchronously.
    fn allocate(&mut self) -> Option<Entity> {
        match self.recycled.pop() {
            Some(entity) => {
                *self.recycled_len.get_mut() -= 1;
                Some(entity)
            }
            None => {
                let current_id = *self.next_id_to_allocate.get_mut();
                let new_next_id_to_allocate = current_id.checked_add(1)?;

                *self.next_id_to_allocate.get_mut() = new_next_id_to_allocate;
                Some(Entity::with_id(current_id))
            }
        }
    }

    /// Tries to allocate an `Entity` atomically.
    fn allocate_atomic(&self) -> Option<Entity> {
        match atomic_decrement_usize(&self.recycled_len) {
            Some(recycled_len) => Some(self.recycled[recycled_len - 1]),
            None => atomic_increment_u32(&self.next_id_to_allocate).map(Entity::with_id),
        }
    }

    /// Tries to recycle the given `Entity`.
    fn deallocate(&mut self, entity: Entity) {
        if let Some(next_version_id) = entity.version().id().checked_add(1) {
            let next_version_id = unsafe { NonZeroU32::new_unchecked(next_version_id) };
            self.recycled.push(Entity::new(entity.id(), Version::new(next_version_id)));
            *self.recycled_len.get_mut() += 1;
        }
    }

    /// Removes all allocated entities from the `recycled` vector and returns an iterator over all
    /// the entities allocated since the last call to `maintain`.
    fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let remaining = *self.recycled_len.get_mut();
        *self.recycled_len.get_mut() = self.recycled.len();

        let new_id_range = self.last_maintained_id..*self.next_id_to_allocate.get_mut();
        self.last_maintained_id = *self.next_id_to_allocate.get_mut();

        self.recycled.drain(remaining..).chain(new_id_range.into_iter().map(Entity::with_id))
    }
}

/// Like `fetch_sub`, but returns `None` on underflow instead of wrapping.
fn atomic_decrement_usize(value: &AtomicUsize) -> Option<usize> {
    let mut prev = value.load(Ordering::Relaxed);

    while prev != 0 {
        match value.compare_exchange_weak(prev, prev - 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(prev) => return Some(prev),
            Err(next_prev) => prev = next_prev,
        }
    }

    None
}

/// Like `fetch_add`, but returns `None` on overflow instead of wrapping.
fn atomic_increment_u32(value: &AtomicU32) -> Option<u32> {
    let mut prev = value.load(Ordering::Relaxed);

    while prev != u32::MAX {
        match value.compare_exchange_weak(prev, prev + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(prev) => return Some(prev),
            Err(next_prev) => prev = next_prev,
        }
    }

    None
}
