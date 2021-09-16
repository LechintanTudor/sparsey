use crate::storage::{Entity, IndexEntity, SparseArray};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Sparse set-based storage for entities.
#[derive(Default)]
pub struct EntityStorage {
    storage: EntitySparseSet,
    allocator: EntityAllocator,
}

impl EntityStorage {
    /// Creates a new `Entity` and returns it.
    pub fn create(&mut self) -> Entity {
        self.maintain();

        let entity = self
            .allocator
            .allocate()
            .expect("No entities left to allocate");

        self.storage.insert(entity);
        entity
    }

    /// Atomically creates a new `Entity` and returns it.
    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("No entities left to allocate")
    }

    /// Removes `entity` from the storage if it exits. Returns whether or not
    /// there was anything to remove.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        self.maintain();

        if self.storage.remove(entity) {
            self.allocator.deallocate(entity);
            true
        } else {
            false
        }
    }

    /// Returns `true` if the storage contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of entities in the storage.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the storage is empty.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Removes all entities from the storage.
    pub fn clear(&mut self) {
        self.storage.clear();
        self.allocator.clear();
    }

    /// Adds the entities created atomically to the storage.
    pub fn maintain(&mut self) {
        for entity in self.allocator.maintain() {
            self.storage.insert(entity);
        }
    }
}

impl AsRef<[Entity]> for EntityStorage {
    fn as_ref(&self) -> &[Entity] {
        &self.storage.entities
    }
}

#[derive(Clone, Default, Debug)]
struct EntitySparseSet {
    sparse: SparseArray,
    entities: Vec<Entity>,
}

impl EntitySparseSet {
    fn insert(&mut self, entity: Entity) {
        let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

        match index_entity {
            Some(index_entity) => unsafe {
                *self.entities.get_unchecked_mut(index_entity.index()) = entity;
            },
            None => {
                *index_entity = Some(IndexEntity::new(
                    self.entities.len() as u32,
                    entity.version(),
                ));
                self.entities.push(entity);
            }
        }
    }

    fn remove(&mut self, entity: Entity) -> bool {
        let dense_index = match self.sparse.remove(entity) {
            Some(dense_index) => dense_index,
            None => return false,
        };

        self.entities.swap_remove(dense_index);

        if let Some(entity) = self.entities.get(dense_index) {
            let new_index_entity = IndexEntity::new(dense_index as u32, entity.version());

            unsafe {
                *self.sparse.get_unchecked_mut(entity.index()) = Some(new_index_entity);
            }
        }

        true
    }

    fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    fn clear(&mut self) {
        self.sparse.clear();
        self.entities.clear();
    }
}

#[derive(Default, Debug)]
struct EntityAllocator {
    current_id: AtomicU32,
    last_id: u32,
    recycled: Vec<Entity>,
    recycled_len: AtomicUsize,
}

impl EntityAllocator {
    fn allocate(&mut self) -> Option<Entity> {
        match self.recycled.pop() {
            Some(entity) => {
                *self.recycled_len.get_mut() -= 1;
                Some(entity)
            }
            None => {
                let current_id = *self.current_id.get_mut();
                *self.current_id.get_mut() = self.current_id.get_mut().checked_add(1)?;
                Some(Entity::with_index(current_id))
            }
        }
    }

    fn allocate_atomic(&self) -> Option<Entity> {
        match atomic_decrement_usize(&self.recycled_len) {
            Some(recycled_len) => Some(self.recycled[recycled_len - 1]),
            None => atomic_increment_u32(&self.current_id).map(Entity::with_index),
        }
    }

    fn deallocate(&mut self, entity: Entity) {
        if let Some(next_entity) = entity.with_next_version() {
            self.recycled.push(next_entity);
            *self.recycled_len.get_mut() += 1;
        }
    }

    fn clear(&mut self) {
        *self.current_id.get_mut() = 0;
        self.last_id = 0;
        self.recycled.clear();
        *self.recycled_len.get_mut() = 0;
    }

    fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let remaining = *self.recycled_len.get_mut();
        *self.recycled_len.get_mut() = self.recycled.len();

        let new_id_range = self.last_id..*self.current_id.get_mut();
        self.last_id = *self.current_id.get_mut();

        self.recycled
            .drain(remaining..)
            .chain(new_id_range.into_iter().map(Entity::with_index))
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
