use crate::{Component, GenericStorage, Storage};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

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

pub struct World {
    storages: HashMap<ComponentTypeId, StorageWrapper>,
}
