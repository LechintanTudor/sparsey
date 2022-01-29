use crate::resources::{Res, ResMut, Resource, SyncResources, UnsafeResources};
use std::any::TypeId;
use std::marker::PhantomData;

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub struct Resources {
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

    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow::<T>().map(Res::new) }
    }

    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        unsafe { self.resources.borrow_mut::<T>().map(ResMut::new) }
    }
}
