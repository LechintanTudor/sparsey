use crate::resources::{Res, ResMut, Resource, ResourceTypeId};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

#[derive(Default)]
pub struct UnsafeResources {
    values: HashMap<ResourceTypeId, AtomicRefCell<Box<dyn Resource>>>,
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
        self.values
            .insert(ResourceTypeId::of::<T>(), AtomicRefCell::new(resource))
            .map(|res| match res.into_inner().downcast::<T>() {
                Ok(res) => res,
                Err(_) => unreachable_unchecked(),
            })
    }

    pub unsafe fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: Resource,
    {
        self.remove_abstract(&ResourceTypeId::of::<T>())
            .map(|res| match res.downcast::<T>() {
                Ok(res) => res,
                Err(_) => unreachable_unchecked(),
            })
    }

    pub unsafe fn remove_abstract(
        &mut self,
        type_id: &ResourceTypeId,
    ) -> Option<Box<dyn Resource>> {
        self.values.remove(type_id).map(|res| res.into_inner())
    }

    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.contains_type_id(&ResourceTypeId::of::<T>())
    }

    pub fn contains_type_id(&self, type_id: &ResourceTypeId) -> bool {
        self.values.contains_key(type_id)
    }

    pub unsafe fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        self.values.get(&ResourceTypeId::of::<T>()).map(|res| {
            Res::new(AtomicRef::map(res.borrow(), |res| {
                match res.as_ref().downcast_ref::<T>() {
                    Some(res) => res,
                    None => unreachable_unchecked(),
                }
            }))
        })
    }

    pub unsafe fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        self.values.get(&ResourceTypeId::of::<T>()).map(|res| {
            ResMut::new(AtomicRefMut::map(res.borrow_mut(), |res| {
                match res.as_mut().downcast_mut::<T>() {
                    Some(res) => res,
                    None => unreachable_unchecked(),
                }
            }))
        })
    }

    pub unsafe fn borrow_abstract(&self, type_id: &ResourceTypeId) -> Option<Res<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| Res::new(AtomicRef::map(res.borrow(), |res| res.as_ref())))
    }

    pub unsafe fn borrow_abstract_mut(
        &self,
        type_id: &ResourceTypeId,
    ) -> Option<ResMut<dyn Resource>> {
        self.values
            .get(type_id)
            .map(|res| ResMut::new(AtomicRefMut::map(res.borrow_mut(), |res| res.as_mut())))
    }
}
