use crate::data::{Entity, IndexEntity, SparseArray};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Default)]
pub struct Entities {
    storage: EntitySparseSet,
    allocator: EntityAllocator,
}

impl Entities {
    pub fn create(&mut self) -> Entity {
        self.maintain();

        let entity = self
            .allocator
            .allocate()
            .expect("No entities left to allocate");

        self.storage.insert(entity);
        entity
    }

    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("No entities left to allocate")
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        self.maintain();

        if self.storage.remove(entity) {
            self.allocator.deallocate(entity);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
        self.allocator.clear();
    }

    pub fn maintain(&mut self) {
        for entity in self.allocator.maintain() {
            self.storage.insert(entity);
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.storage.dense.iter().copied()
    }
}

impl AsRef<[Entity]> for Entities {
    fn as_ref(&self) -> &[Entity] {
        &self.storage.dense
    }
}

#[derive(Clone, Default, Debug)]
struct EntitySparseSet {
    sparse: SparseArray,
    dense: Vec<Entity>,
}

impl EntitySparseSet {
    fn insert(&mut self, entity: Entity) {
        let index_entity = self.sparse.get_mut_or_allocate(entity.index());

        match index_entity {
            Some(e) => {
                *e = IndexEntity::new(e.id(), entity.ver());

                unsafe {
                    *self.dense.get_unchecked_mut(e.index()) = entity;
                }
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.dense.len() as u32, entity.ver()));
                self.dense.push(entity);
            }
        }
    }

    fn remove(&mut self, entity: Entity) -> bool {
        (|| -> Option<()> {
            let index_entity = self.sparse.get_index_entity(entity)?;

            let last_index = self.dense.last()?.index();
            self.dense.swap_remove(index_entity.index());

            unsafe {
                *self.sparse.get_unchecked_mut(last_index) = Some(index_entity);
                *self.sparse.get_unchecked_mut(entity.index()) = None;
            }

            Some(())
        })()
        .is_some()
    }

    fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
    }

    fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
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
                Some(Entity::with_id(current_id))
            }
        }
    }

    fn allocate_atomic(&self) -> Option<Entity> {
        match atomic_decrement_usize(&self.recycled_len) {
            Some(recycled_len) => Some(self.recycled[recycled_len - 1]),
            None => {
                let current_id = atomic_increment_u32(&self.current_id)?;
                Some(Entity::with_id(current_id))
            }
        }
    }

    fn deallocate(&mut self, entity: Entity) {
        if let Some(ver) = entity.ver().next() {
            self.recycled.push(Entity::new(entity.id(), ver));
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
            .chain(new_id_range.into_iter().map(|id| Entity::with_id(id)))
    }
}

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
