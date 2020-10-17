use crate::{Component, GenericStorage, Storage};
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

pub struct StorageWrapper {
    storage: Box<dyn GenericStorage>,
}

impl StorageWrapper {
    pub fn downcast_storage<T>(&self) -> Option<&Storage<T>>
    where
        T: Component,
    {
        self.storage.as_any().downcast_ref::<Storage<T>>()
    }

    pub fn downcast_storage_mut<T>(&mut self) -> Option<&mut Storage<T>>
    where
        T: Component,
    {
        self.storage.as_any_mut().downcast_mut::<Storage<T>>()
    }
}

#[derive(Default)]
pub struct World {
    // storages: HashMap<ComponentTypeId, RefCell<StorageWrapper>>,
}

impl World {
    pub fn insert_storage<T>(&mut self)
    where
        T: Component,
    {
        /*
        self.storages.insert(
            TypeId::of::<T>(),
            RefCell::new(StorageWrapper {
                storage: Box::new(Storage::<T>::default()),
            }),
        );
        */
    }
}

impl World {}
