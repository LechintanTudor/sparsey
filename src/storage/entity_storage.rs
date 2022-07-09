use crate::storage::{Entity, IndexEntity, SparseArray, Version};
use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

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

    /// Removes all entities from the storage and resets the entity allocator.
    pub(crate) fn reset(&mut self) {
        self.storage.clear();
        self.allocator.reset();
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

#[derive(Debug, Default)]
struct EntityAllocator {
    /// Value of the next Entity id to be allocated, capped at (u32::MAX + 1)
    next_id_to_allocate: AtomicU64,
    /// Value of next_id_to_allocate before the last call to maintain
    last_maintained_id: u32,
    /// Entities with the same id as previously deallocated entities, but higher version
    recycled: VecDeque<Entity>,
    /// Number of recycled entities since the last call to maintain
    recycled_since_maintain: AtomicUsize,
}

impl EntityAllocator {
    /// Allocates an entity.
    fn allocate(&mut self) -> Option<Entity> {
        let recycled_since_maintain = *self.recycled_since_maintain.get_mut();

        if recycled_since_maintain < self.recycled.len() {
            *self.recycled_since_maintain.get_mut() += 1;
            Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
        } else {
            if *self.next_id_to_allocate.get_mut() <= (u32::MAX as u64) {
                let id = *self.next_id_to_allocate.get_mut() as u32;
                *self.next_id_to_allocate.get_mut() += 1;
                Some(Entity::with_id(id))
            } else {
                None
            }
        }
    }

    /// Allocates an entity without needing exclusive access over the allocator. Slower than
    /// `allocate`.
    fn allocate_atomic(&self) -> Option<Entity> {
        match increment_recycled_since_maintain(&self.recycled_since_maintain, self.recycled.len())
        {
            Some(recycled_since_maintain) => {
                Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
            }
            None => increment_next_id_to_allocate(&self.next_id_to_allocate).map(Entity::with_id),
        }
    }

    /// Deallocates the entity and attempts to recycle its id.
    fn deallocate(&mut self, entity: Entity) {
        let next_version_id = NonZeroU32::new(entity.version().id().wrapping_add(1));

        if let Some(next_version_id) = next_version_id {
            self.recycled.push_front(Entity::new(entity.id(), Version::new(next_version_id)));
        }
    }

    /// Clears the recycled entities queue and returns an iterator over all allocated entities
    /// since the last call to maintain.
    fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let recycled_range = {
            let recycled_since_maintain = *self.recycled_since_maintain.get_mut();
            *self.recycled_since_maintain.get_mut() = 0;
            (self.recycled.len() - recycled_since_maintain)..
        };

        let new_id_range = if *self.next_id_to_allocate.get_mut() <= (u32::MAX as u64) {
            let next_id_to_allocate = *self.next_id_to_allocate.get_mut() as u32;
            let new_id_range = self.last_maintained_id..next_id_to_allocate;
            self.last_maintained_id = next_id_to_allocate;
            new_id_range
        } else {
            0..0
        };

        self.recycled.drain(recycled_range).chain(new_id_range.map(Entity::with_id))
    }

    /// Resets the allocator to its default state without freeing the allocated memory.
    fn reset(&mut self) {
        *self.next_id_to_allocate.get_mut() = 0;
        self.last_maintained_id = 0;
        self.recycled.clear();
        *self.recycled_since_maintain.get_mut() = 0;
    }
}

/// Atomically increments `recycled_since_maintain`, capping at `recycled_len`.
/// Returns the value before the increment if it is <= `recycled_len`.
fn increment_recycled_since_maintain(
    recycled_since_maintain: &AtomicUsize,
    recycled_len: usize,
) -> Option<usize> {
    let mut prev = recycled_since_maintain.load(Ordering::Relaxed);

    while prev < recycled_len {
        match recycled_since_maintain.compare_exchange_weak(
            prev,
            prev + 1,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(prev) => return Some(prev),
            Err(next_prev) => prev = next_prev,
        }
    }

    None
}

/// Atomically increments `next_id_to_allocate`, capping at (`u32::MAX + 1`).
/// Returns the value before the increment if it is <= `u32::MAX`
fn increment_next_id_to_allocate(next_id_to_allocate: &AtomicU64) -> Option<u32> {
    let mut prev = next_id_to_allocate.load(Ordering::Relaxed);

    while prev <= (u32::MAX as u64) {
        match next_id_to_allocate.compare_exchange_weak(
            prev,
            prev + 1,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(prev) => return Some(prev as u32),
            Err(next_prev) => prev = next_prev,
        }
    }

    None
}
