mod entity;
mod entity_allocator;
mod entity_manager;
mod entity_sparse_set;
mod sparse_vec;

pub use self::entity::*;
pub use self::entity_allocator::*;
pub use self::entity_manager::*;
pub use self::entity_sparse_set::*;
pub use self::sparse_vec::*;

#[derive(Debug)]
pub struct EntityStorage {
    entities: EntityManager,
}

impl EntityStorage {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entities: EntityManager::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn create(&mut self) -> Entity {
        self.entities
            .create()
            .expect("Failed to create a new Entity")
    }

    #[inline]
    #[must_use]
    pub fn create_atomic(&self) -> Entity {
        self.entities
            .create_atomic()
            .expect("Failed to create a new Entity")
    }

    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        self.entities.destroy(entity)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.entities.reset();
    }

    #[inline]
    pub fn maintain(&mut self) {
        self.entities.maintain();
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.entities()
    }
}
