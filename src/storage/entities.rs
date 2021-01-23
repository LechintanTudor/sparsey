use crate::storage::{Entity, IndexEntity, SparseArray};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Default)]
pub struct EntityStorage {
    allocator: EntityAllocator,
    entities: EntitySparseSet,
}

impl EntityStorage {
    pub fn create(&mut self) -> Entity {
        let entity = self
            .allocator
            .allocate()
            .expect("No entities left to be created");

        self.entities.insert(entity);
        entity
    }

    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("No entities left to be created")
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if self.entities.remove(entity) {
            self.allocator.remove(entity);
            true
        } else {
            false
        }
    }

    pub fn maintain(&mut self) {}
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
                *e = IndexEntity::new(e.id(), entity.gen());

                unsafe {
                    *self.dense.get_unchecked_mut(e.index()) = entity;
                }
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.dense.len() as u32, entity.gen()));
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
}

#[derive(Default, Debug)]
struct EntityAllocator {
    current_entity_id: AtomicU32,
    removed_entities: Vec<Entity>,
    removed_index: AtomicUsize,
}

impl EntityAllocator {
    fn allocate(&mut self) -> Option<Entity> {
        self.maintain();

        match self.removed_entities.pop() {
            Some(entity) => Some(Entity::new(entity.id(), entity.gen().next()?)),
            None => {
                let entity = Entity::with_id(*self.current_entity_id.get_mut());
                *self.current_entity_id.get_mut() += 1;
                Some(entity)
            }
        }
    }

    fn allocate_atomic(&self) -> Option<Entity> {
        atomic_decrement_usize(&self.removed_index)
            .map(|i| unsafe { *self.removed_entities.get_unchecked(i) })
    }

    fn remove(&mut self, entity: Entity) {
        self.maintain();
        self.removed_entities.push(entity)
    }

    fn maintain(&mut self) {
        self.removed_entities
            .truncate(self.removed_entities.len() - *self.removed_index.get_mut());

        *self.removed_index.get_mut() = self.removed_entities.len();
    }
}

fn atomic_decrement_usize(value: &AtomicUsize) -> Option<usize> {
    let mut prev = value.load(Ordering::Relaxed);

    while prev != 0 {
        match value.compare_exchange_weak(prev, prev - 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(value) => return Some(value),
            Err(value) => prev = value,
        }
    }

    None
}
