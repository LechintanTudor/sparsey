use crate::resources::Resource;
use crate::utils::{ChangeTicks, UnsafeUnwrap};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::IterMut as HashMapIterMut;

/// Container for a type-erased `Resource` and its `ChangeTicks`.
pub struct ResourceCell {
    value: Box<dyn Resource>,
    pub(crate) ticks: ChangeTicks,
}

impl ResourceCell {
    pub(crate) fn new(value: Box<dyn Resource>, ticks: ChangeTicks) -> Self {
        Self { value, ticks }
    }

    /// Returns a type-erased reference to the resource stored inside.
    pub fn value(&self) -> &dyn Resource {
        &*self.value
    }

    pub(crate) fn value_mut(&mut self) -> &mut dyn Resource {
        &mut *self.value
    }

    /// Returns the resource's `ChangeTicks`.
    pub fn ticks(&self) -> ChangeTicks {
        self.ticks
    }
}

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub(crate) struct ResourceStorage {
    resources: FxHashMap<TypeId, AtomicRefCell<ResourceCell>>,
}

impl ResourceStorage {
    pub fn insert<T>(&mut self, resource: T, ticks: ChangeTicks) -> Option<T>
    where
        T: Resource,
    {
        let cell = ResourceCell::new(Box::new(resource), ticks);

        self.resources
            .insert(TypeId::of::<T>(), AtomicRefCell::new(cell))
            .map(|c| unsafe { *c.into_inner().value.downcast().unsafe_unwrap() })
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(|c| unsafe { *c.into_inner().value.downcast().unsafe_unwrap() })
    }

    pub fn contains(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains_key(resource_type_id)
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    pub fn borrow(&self, resource_type_id: &TypeId) -> Option<AtomicRef<ResourceCell>> {
        self.resources
            .get(resource_type_id)
            .map(AtomicRefCell::borrow)
    }

    pub fn borrow_mut(&self, resource_type_id: &TypeId) -> Option<AtomicRefMut<ResourceCell>> {
        self.resources
            .get(resource_type_id)
            .map(AtomicRefCell::borrow_mut)
    }

    pub fn iter(&mut self) -> ResourceStorageIter {
        ResourceStorageIter::new(self)
    }
}

/// Iterator over the resources in a `World`.
pub struct ResourceStorageIter<'a> {
    inner: HashMapIterMut<'a, TypeId, AtomicRefCell<ResourceCell>>,
}

impl<'a> ResourceStorageIter<'a> {
    fn new(resource_storage: &'a mut ResourceStorage) -> Self {
        Self {
            inner: resource_storage.resources.iter_mut(),
        }
    }
}

impl<'a> Iterator for ResourceStorageIter<'a> {
    type Item = (&'a TypeId, &'a ResourceCell);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(type_id, cell)| (type_id, &*cell.get_mut()))
    }
}
