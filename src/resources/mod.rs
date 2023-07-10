//! Resource creation and management.

mod res;
mod resource;
mod sync_resources;
mod unsafe_resources;

pub use self::res::*;
pub use self::resource::*;
pub use self::sync_resources::*;
pub(crate) use self::unsafe_resources::*;
use std::any::TypeId;
use std::fmt;
use std::marker::PhantomData;

/// Container for resources.
#[derive(Default)]
pub struct Resources {
    resources: UnsafeResources,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    /// Returns a `Send + Sync` view over all resources that can be accessed from any thread.
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

    /// Borrows a view over a resource. Panics if the resource does not exist.
    pub fn borrow<T>(&self) -> Res<T>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow() }
    }

    /// Borrows a mutable view over a resource. Panics if the resource does not exist.
    pub fn borrow_mut<T>(&self) -> ResMut<T>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow_mut() }
    }

    /// Borrows a view over a resource if that resource exists.
    pub fn try_borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.try_borrow() }
    }

    /// Borrows a mutable view over a resource if that resource exists.
    pub fn try_borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.try_borrow_mut() }
    }

    /// Returns `true` if the stoage contains a resource with type `T`.
    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.resources.contains::<T>()
    }

    /// Returns `true` if the storage contains a resource with the given [`TypeId`].
    #[inline]
    #[must_use]
    pub fn contains_type_id(&self, resource_type_id: TypeId) -> bool {
        self.resources.contains_type_id(resource_type_id)
    }

    /// Returns the number of resources in the storage.
    #[inline]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns `true` if the storage contains no resources.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Removes a resource from the storage and returns it.
    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        unsafe { self.resources.remove() }
    }

    /// Deletes the resource with the given [`TypeId`] from the storage. Returns `true` if there was
    /// anything to delete.
    #[inline]
    pub fn delete(&mut self, resource_type_id: TypeId) -> bool {
        unsafe { self.resources.delete(resource_type_id) }
    }

    /// Removes all resources from the storage.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { self.resources.clear() }
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resources")
            .field(
                "resource_type_ids",
                &DebugResourceTypeIdSet(&self.resources),
            )
            .finish_non_exhaustive()
    }
}
