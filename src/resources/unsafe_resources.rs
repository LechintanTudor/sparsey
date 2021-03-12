use crate::data::{AtomicRef, AtomicRefCell, AtomicRefMut};
use crate::resources::{Res, ResMut, Resource};
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

#[derive(Default)]
pub struct UnsafeResources {
    values: HashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
    pub unsafe fn clear(&mut self) {
        self.values.clear();
    }

    pub unsafe fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
    where
        T: Resource,
    {
        self.insert_boxed(Box::new(resource))
    }

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

    pub unsafe fn insert_dyn(
        &mut self,
        type_id: TypeId,
        resource: Box<dyn Resource>,
    ) -> Option<Box<dyn Resource>> {
        self.values
            .insert(type_id, AtomicRefCell::new(resource))
            .map(|res| res.into_inner())
    }

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

    pub unsafe fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
        self.values.remove(type_id).map(|res| res.into_inner())
    }

    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.values.contains_key(type_id)
    }

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

    pub unsafe fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| Res::new(AtomicRef::map(res.borrow(), Box::as_ref)))
    }

    pub unsafe fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| ResMut::new(AtomicRefMut::map(res.borrow_mut(), Box::as_mut)))
    }
}
