use crate::{AtomicRefCell, Ref, RefMut, Storage};
use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct World {
    storages: HashMap<TypeId, AtomicRefCell<Box<dyn Storage>>>,
}

impl World {
    pub fn borrow<S>(&self) -> Option<Ref<S>>
    where
        S: Storage,
    {
        self.storages
            .get(&TypeId::of::<S>())
            .map(|s| s.borrow().map(|s| s.as_any().downcast_ref::<S>().unwrap()))
    }

    pub fn borrow_mut<S>(&self) -> Option<RefMut<S>>
    where
        S: Storage,
    {
        self.storages.get(&TypeId::of::<S>()).map(|s| {
            s.borrow_mut()
                .map(|s| s.as_any_mut().downcast_mut::<S>().unwrap())
        })
    }

    pub fn insert<S, B>(&mut self, storage: B)
    where
        S: Storage,
        B: Into<Box<S>>,
    {
        self.storages
            .insert(TypeId::of::<S>(), AtomicRefCell::new(storage.into()));
    }

    pub fn remove<S>(&mut self)
    where
        S: Storage,
    {
        self.storages.remove(&TypeId::of::<S>());
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    /*
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
    */
}
