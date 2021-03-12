use crate::resources::{Res, ResMut, Resource, SyncResources, UnsafeResources};
use std::any::TypeId;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Resources {
    internal: UnsafeResources,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    pub fn sync<'a>(&'a self) -> SyncResources<'a> {
        SyncResources::new(&self.internal)
    }

    pub unsafe fn internal(&self) -> &UnsafeResources {
        &self.internal
    }

    pub fn clear(&mut self) {
        unsafe {
            self.internal.clear();
        }
    }

    pub fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.insert(resource) }
    }

    pub fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.insert_boxed(resource) }
    }

    pub unsafe fn insert_dyn(
        &mut self,
        type_id: TypeId,
        resource: Box<dyn Resource>,
    ) -> Option<Box<dyn Resource>> {
        self.internal.insert_dyn(type_id, resource)
    }

    pub fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.remove::<T>() }
    }

    pub fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
        unsafe { self.internal.remove_dyn(type_id) }
    }

    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.internal.contains(type_id)
    }

    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.borrow() }
    }

    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.borrow_mut() }
    }

    pub fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
        unsafe { self.internal.borrow_dyn(type_id) }
    }

    pub fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
        unsafe { self.internal.borrow_dyn_mut(type_id) }
    }
}
