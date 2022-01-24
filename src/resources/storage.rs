use crate::resources::{Resource, SyncResources, UnsafeResources};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;
use std::marker::PhantomData;

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub(crate) struct Resources {
    resources: UnsafeResources,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    pub fn sync(&self) -> SyncResources {
        SyncResources::new(&self.resources)
    }

    pub fn insert<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        unsafe { self.resources.insert(resource) }
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        unsafe { self.resources.remove::<T>() }
    }

    pub fn delete(&mut self, resource_type_id: &TypeId) -> bool {
        unsafe { self.resources.delete(resource_type_id) }
    }

    pub fn contains(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains(resource_type_id)
    }

    pub fn clear(&mut self) {
        unsafe { self.resources.clear() }
    }

    pub fn borrow<T>(&self) -> Option<AtomicRef<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow::<T>() }
    }

    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow_mut::<T>() }
    }
}
