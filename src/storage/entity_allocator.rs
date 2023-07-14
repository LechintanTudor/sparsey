use crate::prelude::Entity;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Allocates and recycles entities.
#[derive(Default, Debug)]
pub struct EntityAllocator {
    /// Value of the next Entity index to be allocated, capped at (u32::MAX + 1)
    next_index_to_allocate: AtomicU64,
    /// Value of next_index_to_allocate before the last call to maintain
    last_maintained_index: u64,
    /// Entities with the same index as previously deallocated entities, but higher version
    recycled: VecDeque<Entity>,
    /// Number of recycled entities since the last call to maintain
    recycled_since_maintain: AtomicUsize,
}

impl EntityAllocator {
    /// Allocates an entity synchronously.
    pub fn allocate(&mut self) -> Option<Entity> {
        let recycled_since_maintain = *self.recycled_since_maintain.get_mut();

        if recycled_since_maintain < self.recycled.len() {
            *self.recycled_since_maintain.get_mut() += 1;
            Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
        } else if *self.next_index_to_allocate.get_mut() <= (u32::MAX as u64) {
            let index = *self.next_index_to_allocate.get_mut() as u32;
            *self.next_index_to_allocate.get_mut() += 1;
            Some(Entity::with_index(index))
        } else {
            None
        }
    }

    /// Allocates an entity without needing exclusive access over the allocator. Slower than
    /// `allocate`.
    pub fn allocate_atomic(&self) -> Option<Entity> {
        match increment_recycled_since_maintain(&self.recycled_since_maintain, self.recycled.len())
        {
            Some(recycled_since_maintain) => {
                Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
            }
            None => {
                increment_next_index_to_allocate(&self.next_index_to_allocate)
                    .map(Entity::with_index)
            }
        }
    }

    /// Deallocates the entity and attempts to recycle its index.
    pub fn recycle(&mut self, entity: Entity) {
        if let Some(next_version) = entity.version().next() {
            self.recycled
                .push_front(Entity::new(entity.index(), next_version));
        }
    }

    /// Clears the recycled entities queue and returns an iterator over all allocated entities
    /// since the last call to maintain.
    pub fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let recycled_range = {
            let recycled_since_maintain = *self.recycled_since_maintain.get_mut();
            *self.recycled_since_maintain.get_mut() = 0;
            (self.recycled.len() - recycled_since_maintain)..
        };

        let new_index_range = {
            let next_index_to_allocate = *self.next_index_to_allocate.get_mut();
            let new_index_range = self.last_maintained_index..next_index_to_allocate;
            self.last_maintained_index = next_index_to_allocate;
            new_index_range
        };

        self.recycled
            .drain(recycled_range)
            .chain(new_index_range.map(|i| {
                // next_index_to_allocate is capped at (u32::MAX + 1), so i <= u32::MAX
                Entity::with_index(i as u32)
            }))
    }

    /// Resets the allocator to its default state without freeing the allocated memory.
    pub fn reset(&mut self) {
        *self.next_index_to_allocate.get_mut() = 0;
        self.last_maintained_index = 0;
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

/// Atomically increments `next_index_to_allocate`, capping at (`u32::MAX + 1`).
/// Returns the value before the increment if it is <= `u32::MAX`
fn increment_next_index_to_allocate(next_index_to_allocate: &AtomicU64) -> Option<u32> {
    let mut prev = next_index_to_allocate.load(Ordering::Relaxed);

    while prev <= (u32::MAX as u64) {
        match next_index_to_allocate.compare_exchange_weak(
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
