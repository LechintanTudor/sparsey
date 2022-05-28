use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use std::any::TypeId;

/// View over thread-safe resources.
#[derive(Clone, Copy)]
pub struct SyncResources<'a> {
    resources: &'a UnsafeResources,
}

unsafe impl Send for SyncResources<'_> {}
unsafe impl Sync for SyncResources<'_> {}

impl<'a> SyncResources<'a> {
    pub(crate) fn new(resources: &'a UnsafeResources) -> Self {
        Self { resources }
    }

    /// Borrows a view over a resource. Panics if the resource does not exist.
    pub fn borrow<T>(&self) -> Res<'a, T>
    where
        T: Resource + Sync,
    {
        unsafe { self.resources.borrow() }
    }

    /// Borrows a mutable view over a resource. Panics if the resource does not exist.
    pub fn borrow_mut<T>(&self) -> ResMut<'a, T>
    where
        T: Resource + Send,
    {
        unsafe { self.resources.borrow_mut() }
    }

    /// Borrows a view over a resource if that resource exists.
    pub fn try_borrow<T>(&self) -> Option<Res<'a, T>>
    where
        T: Resource + Sync,
    {
        unsafe { self.resources.try_borrow() }
    }

    /// Borrows a mutable view over a resource if that resource exists.
    pub fn try_borrow_mut<T>(&self) -> Option<ResMut<'a, T>>
    where
        T: Resource + Send,
    {
        unsafe { self.resources.try_borrow_mut() }
    }

    /// Returns `true` if the stoage contains a resource with type `T`.
    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.resources.contains::<T>()
    }

    /// Returns `true` if the storage contains a resource with the given `TypeId`.
    #[inline]
    pub fn contains_type_id(&self, resource_type_id: TypeId) -> bool {
        self.resources.contains_type_id(resource_type_id)
    }

    /// Returns the number of resources in the storage.
    #[inline]
    pub fn len(&self) -> usize {
        self.resources.len()
    }
}
