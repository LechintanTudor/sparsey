use crate::{
    data::view::StorageView,
    entity::Entity,
    registry::{BorrowFromWorld, World},
    storage::{SparseArray, SparseSet, SparseSetView},
};
use std::ops::Deref;

#[derive(Default, Debug)]
struct EntityAllocator {
    index: usize,
    removed: Vec<Entity>,
}

impl EntityAllocator {
    fn allocate(&mut self) -> Entity {
        match self.removed.pop() {
            Some(entity) => Entity::from_id_and_gen(entity.id(), entity.gen() + 1),
            None => {
                let index = self.index;
                self.index += 1;
                Entity::new(index as u32)
            }
        }
    }

    fn remove(&mut self, entity: Entity) {
        self.removed.push(entity)
    }
}

#[derive(Default)]
pub struct EntityStorage {
    allocator: EntityAllocator,
    entities: SparseSet<()>,
}

impl EntityStorage {
    pub fn create(&mut self) -> Entity {
        let entity = self.allocator.allocate();
        self.entities.insert(entity, ());
        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        if self.entities.remove(entity).is_some() {
            self.allocator.remove(entity)
        }
    }
}

pub struct Entities<'a> {
    storage: &'a EntityStorage,
}

impl Deref for Entities<'_> {
    type Target = EntityStorage;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a> BorrowFromWorld<'a> for Entities<'a> {
    fn borrow(world: &'a World) -> Self {
        Self {
            storage: world.entities(),
        }
    }
}

impl<'a> StorageView<'a> for &'a Entities<'a> {
    const STRICT: bool = true;
    type Output = &'a Entity;
    type Component = &'a Entity;
    type Data = *const Entity;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, _) = self.entities.split();
        (sparse, dense, dense.as_ptr())
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        &*data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        let index = self.entities.sparse().get(entity)?.index();
        Some(self.entities.dense().get_unchecked(index))
    }
}
