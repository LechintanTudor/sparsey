use crate::data::{AtomicRef, AtomicRefCell, AtomicRefMut};
use crate::resources::{Res, ResMut, Resource};
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

/// Maps `TypeIds` to type erased resources. Allows interior
/// mutability. Unsafe because the struct itself is `Send` and `Sync` but
/// doesn't ensure the resources it holds are `Send` and `Sync` themselves.
#[derive(Default)]
pub struct UnsafeResources {
    values: HashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
    /// Remove all resources.
    pub unsafe fn clear(&mut self) {
        self.values.clear();
    }

    /// Insert a resource and return the previous one, if any.
    pub unsafe fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
    where
        T: Resource,
    {
        self.insert_boxed(Box::new(resource))
    }

    /// Insert a boxed resource and return the previous one, if any.
    pub unsafe fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
    where
        T: Resource,
    {
        self.insert_dyn(TypeId::of::<T>(), resource)
            .map(|res| match res.downcast() {
                Ok(res) => res,
                Err(_) => unreachable_unchecked(),
            })
    }

    /// Insert a type erased resource at the given `TypeId`.
    /// The `TypeId` of the resource must be the same as the one passed.
    pub unsafe fn insert_dyn(
        &mut self,
        type_id: TypeId,
        resource: Box<dyn Resource>,
    ) -> Option<Box<dyn Resource>> {
        self.values
            .insert(type_id, AtomicRefCell::new(resource))
            .map(|res| res.into_inner())
    }

    /// Remove a resource and return it if it was successfully removed.
    pub unsafe fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: Resource,
    {
        self.remove_dyn(&TypeId::of::<T>())
            .map(|res| match res.downcast::<T>() {
                Ok(res) => res,
                Err(_) => unreachable_unchecked(),
            })
    }

    /// Remove the resource at the given `TypeId` and return it if it was successfully removed.
    pub unsafe fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
        self.values.remove(type_id).map(|res| res.into_inner())
    }

    /// Check if the set contains a resource at the given `TypeId`.
    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.values.contains_key(type_id)
    }

    /// Get the number of resources in the set.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Get a shared borrow of a resource if it exists.
    pub unsafe fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        self.borrow_dyn(&TypeId::of::<T>()).map(|res| {
            Res::map(res, |res| match res.downcast_ref::<T>() {
                Some(res) => res,
                None => unreachable_unchecked(),
            })
        })
    }

    /// Get an exclusive borrow of a resource if it exists.
    pub unsafe fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        self.borrow_dyn_mut(&TypeId::of::<T>()).map(|res| {
            ResMut::map(res, |res| match res.downcast_mut::<T>() {
                Some(res) => res,
                None => unreachable_unchecked(),
            })
        })
    }

    /// Get a shared borrow of the resource at the given `TypeId`, if it exists.
    pub unsafe fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| Res::new(AtomicRef::map(res.borrow(), Box::as_ref)))
    }

    /// Get an exclusive borrow of the resource at the given `TypeId`, if it exists.
    pub unsafe fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| ResMut::new(AtomicRefMut::map(res.borrow_mut(), Box::as_mut)))
    }
}
