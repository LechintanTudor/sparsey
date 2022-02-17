use crate::resources::{Res, ResMut, Resource, SyncResources, UnsafeResources};
use std::any::TypeId;
use std::marker::PhantomData;

/// Container for resources.
#[derive(Default)]
pub struct Resources {
    resources: UnsafeResources,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    /// Returns a `Send + Sync` view over all resources that can be accessed from other threads.
    pub fn sync(&self) -> SyncResources {
        SyncResources::new(&self.resources)
    }

    /// Adds a resource of type `T` to the storage and returns the previous one, if any.
    pub fn insert<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        unsafe { self.resources.insert(resource) }
    }

    /// Removes a resource from the storage and returns it.
    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        unsafe { self.resources.remove::<T>() }
    }

    /// Deletes the resource with the given `TypeId` from the storage. Returns `true` if there was
    /// anything to delete.
    pub fn delete(&mut self, resource_type_id: &TypeId) -> bool {
        unsafe { self.resources.delete(resource_type_id) }
    }

    /// Returns `true` if the storage contains a resource with the given `TypeId`.
    pub fn contains(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains(resource_type_id)
    }

    /// Removes all resources from the storage.
    pub fn clear(&mut self) {
        unsafe { self.resources.clear() }
    }

    /// Borrows a view over a resource.
    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow::<T>().map(Res::new) }
    }

    /// Mutably borrows a view over a resource.
    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow_mut::<T>().map(ResMut::new) }
    }
}
