mod borrow;
mod component;
mod component_sparse_set;
mod component_storage;
mod entity;
mod entity_allocator;
mod entity_sparse_set;
mod group;
mod group_mask;
mod sparse_vec;

pub use self::borrow::*;
pub use self::component::*;
pub use self::component_sparse_set::*;
pub use self::component_storage::*;
pub use self::entity::*;
pub use self::entity_allocator::*;
pub use self::entity_sparse_set::*;
pub use self::group::*;
pub use self::group_mask::*;
pub use self::sparse_vec::*;

#[derive(Default, Debug)]
pub struct EntityStorage {
    allocator: EntityAllocator,
    entities: EntitySparseSet,
    components: ComponentStorage,
}

impl EntityStorage {
    pub fn register_component<T>(&mut self) -> bool
    where
        T: Component,
    {
        self.components.register::<T>()
    }

    #[inline]
    #[must_use]
    pub fn create(&mut self) -> Entity {
        let entity = self
            .allocator
            .allocate()
            .expect("Failed to create a new Entity");

        self.entities.insert(entity);
        entity
    }

    #[inline]
    #[must_use]
    pub fn create_atomic(&self) -> Entity {
        self.allocator
            .allocate_atomic()
            .expect("Failed to create a new Entity")
    }

    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.allocator.recycle(entity);
        true
    }

    #[inline]
    pub fn clear(&mut self) {
        self.maintain();
        self.entities.clear();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.allocator.reset();
        self.entities.clear();
    }

    #[inline]
    pub fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| {
            self.entities.insert(entity);
        });
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    #[inline]
    #[must_use]
    pub fn borrow_entities(&self) -> Entities {
        Entities::new(self)
    }

    #[must_use]
    pub fn borrow_components<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        self.components.borrow::<T>()
    }

    #[must_use]
    pub fn borrow_components_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        self.components.borrow_mut::<T>()
    }
}
