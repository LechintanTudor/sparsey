use crate::resources::{Res, ResMut, Resource, ResourceTypeId, SyncResources, UnsafeResources};
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

    pub fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: Resource,
    {
        unsafe { self.internal.remove::<T>() }
    }

    pub fn remove_abstract(&mut self, type_id: &ResourceTypeId) -> Option<Box<dyn Resource>> {
        unsafe { self.internal.remove_abstract(type_id) }
    }

    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.internal.contains::<T>()
    }

    pub fn contains_type_id(&self, type_id: &ResourceTypeId) -> bool {
        self.internal.contains_type_id(type_id)
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

    pub fn borrow_abstract(&self, type_id: &ResourceTypeId) -> Option<Res<dyn Resource>> {
        unsafe { self.internal.borrow_abstract(type_id) }
    }

    pub fn borrow_abstract_mut(&self, type_id: &ResourceTypeId) -> Option<ResMut<dyn Resource>> {
        unsafe { self.internal.borrow_abstract_mut(type_id) }
    }
}
