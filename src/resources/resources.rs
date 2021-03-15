use crate::resources::{Res, ResMut, Resource, SyncResources, UnsafeResources};
use std::any::TypeId;
use std::marker::PhantomData;

/// Collection of resources to be used in systems.
#[derive(Default)]
pub struct Resources {
    internal: UnsafeResources,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    /// Get a view of resources which can be safely shared across threads.
    pub fn sync<'a>(&'a self) -> SyncResources<'a> {
        SyncResources::new(&self.internal)
    }

    /// Get the internal data structure used to store the resources.
    pub unsafe fn internal(&self) -> &UnsafeResources {
        &self.internal
    }

    /// Remove all resources.
    pub fn clear(&mut self) {
        unsafe {
            self.internal.clear();
        }
    }

    /// Insert a resource and return the previous one, if any.
    pub fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.insert(resource) }
    }

    /// Insert a boxed resource and return the previous one, if any.
    pub fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.insert_boxed(resource) }
    }

    /// Insert a type erased resource at the given `TypeId`.
    /// The `TypeId` of the resource must be the same as the one passed.
    pub unsafe fn insert_dyn(
        &mut self,
        type_id: TypeId,
        resource: Box<dyn Resource>,
    ) -> Option<Box<dyn Resource>> {
        self.internal.insert_dyn(type_id, resource)
    }

    /// Remove a resource and return it if it was successfully removed.
    pub fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.remove::<T>() }
    }

    /// Remove the resource at the given `TypeId` and return it if it was successfully removed.
    pub fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
        unsafe { self.internal.remove_dyn(type_id) }
    }

    /// Check if the set contains a resource at the given `TypeId`.
    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.internal.contains(type_id)
    }

    /// Get the number of resources in the set.
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    /// Get a shared borrow of a resource if it exists.
    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.borrow() }
    }

    /// Get an exclusive borrow of a resource if it exists.
    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.borrow_mut() }
    }

    /// Get a shared borrow of the resource at the given `TypeId`, if it exists.
    pub fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
        unsafe { self.internal.borrow_dyn(type_id) }
    }

    /// Get an exclusive borrow of the resource at the given `TypeId`, if it exists.
    pub fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
        unsafe { self.internal.borrow_dyn_mut(type_id) }
    }
}
