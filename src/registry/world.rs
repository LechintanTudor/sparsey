use crate::{
    atomic_ref_cell::{AtomicRefCell, Ref, RefMut},
    entity::Entity,
    registry::{borrow::BorrowFromWorld, Component, ComponentSource},
    storage::{Entities, SparseSet, Storage},
};
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Default)]
pub struct World {
    storages: HashMap<ComponentTypeId, AtomicRefCell<Box<dyn Storage>>>,
    entities: Entities,
}

impl World {
    pub fn borrow<T>(&self) -> Option<Ref<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow()
                .map(|s| s.as_any().downcast_ref::<SparseSet<T>>().unwrap())
        })
    }

    pub fn borrow_mut<T>(&self) -> Option<RefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow_mut()
                .map(|s| s.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap())
        })
    }

    pub(crate) fn borrow_raw<T>(&self) -> Option<Ref<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow()
                .map(|s| s.as_any().downcast_ref::<SparseSet<T>>().unwrap())
        })
    }

    pub(crate) fn borrow_raw_mut<T>(&self) -> Option<RefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow_mut()
                .map(|s| s.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap())
        })
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| AtomicRefCell::new(Box::new(SparseSet::<T>::default())));
    }

    pub fn push<'a, C>(&'a mut self, components: C) -> Entity 
    where
        C: ComponentSource<'a>,
    {
        let entity = self.entities.create();
        let mut target = C::Target::borrow(self);
        C::insert(&mut target, entity, components);
        entity
    }

    pub fn remove<'a, C>(&'a mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSource<'a>,   
    {
        let mut target = C::Target::borrow(self);
        C::remove(&mut target, entity)
    }

    pub fn delete<'a, C>(&'a mut self, entity: Entity) 
    where
        C: ComponentSource<'a>,
    {
        let mut target = C::Target::borrow(self);
        C::delete(&mut target, entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let mut world = World::default();
        world.register::<u16>();
        world.register::<u32>();

        let _u16_ref1 = world.borrow::<u16>().unwrap();
        let _u16_ref2 = world.borrow::<u16>().unwrap();

        let _u32_ref1 = world.borrow::<u32>().unwrap();
        let _u32_ref2 = world.borrow::<u32>().unwrap();
    }

    #[test]
    #[should_panic]
    fn borrow_mut() {
        let mut world = World::default();
        world.register::<u16>();

        let _u16_ref1 = world.borrow_mut::<u16>().unwrap();
        let _u16_ref2 = world.borrow_mut::<u16>().unwrap();
    }
}
