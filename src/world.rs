use crate::{AtomicRefCell, Component, GenericStorage, Ref, RefMut, Storage};
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Default)]
pub struct World {
    storages: HashMap<ComponentTypeId, AtomicRefCell<Box<dyn GenericStorage>>>,
}

impl World {
    pub fn borrow<T>(&self) -> Option<Ref<Storage<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow()
                .map(|s| s.as_any().downcast_ref::<Storage<T>>().unwrap())
        })
    }

    pub fn borrow_mut<T>(&self) -> Option<RefMut<Storage<T>>>
    where
        T: Component,
    {
        self.storages.get(&TypeId::of::<T>()).map(|s| {
            s.borrow_mut()
                .map(|s| s.as_any_mut().downcast_mut::<Storage<T>>().unwrap())
        })
    }

    pub fn insert<T, S>(&mut self, storage: S)
    where
        T: Component,
        S: Into<Box<Storage<T>>>,
    {
        self.storages
            .insert(TypeId::of::<T>(), AtomicRefCell::new(storage.into()));
    }

    pub fn remove<T>(&mut self)
    where
        T: Component,
    {
        self.storages.remove(&TypeId::of::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let mut world = World::default();
        world.insert(Storage::<u16>::default());
        world.insert(Storage::<u32>::default());

        let _u16_ref1 = world.borrow::<u16>().unwrap();
        let _u16_ref2 = world.borrow::<u16>().unwrap();

        let _u32_ref1 = world.borrow::<u32>().unwrap();
        let _u32_ref2 = world.borrow::<u32>().unwrap();
    }

    #[test]
    #[should_panic]
    fn borrow_mut() {
        let mut world = World::default();
        world.insert(Storage::<u16>::default());

        let _u16_ref1 = world.borrow_mut::<u16>().unwrap();
        let _u16_ref2 = world.borrow_mut::<u16>().unwrap();
    }
}
