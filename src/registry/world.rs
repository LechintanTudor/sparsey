use crate::{
    atomic_ref_cell::{AtomicRefCell, Ref, RefMut},
    registry::Component,
    storage::{SparseSet, Storage},
};
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Default)]
pub struct World {
    storages: HashMap<ComponentTypeId, AtomicRefCell<Box<dyn Storage>>>,
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

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| AtomicRefCell::new(Box::new(SparseSet::<T>::default())));
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
