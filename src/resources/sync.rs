use atomic_refcell::{AtomicRef, AtomicRefMut};

use crate::resources::{Resource, UnsafeResources};

#[derive(Clone, Copy)]
pub(crate) struct SyncResources<'a> {
    resources: &'a UnsafeResources,
}

unsafe impl Send for SyncResources<'_> {}
unsafe impl Sync for SyncResources<'_> {}

impl<'a> SyncResources<'a> {
    pub fn new(resources: &'a UnsafeResources) -> Self {
        Self { resources }
    }

    pub fn borrow<T>(&self) -> Option<AtomicRef<'a, T>>
    where
        T: Resource + Sync,
    {
        unsafe { self.resources.borrow::<T>() }
    }

    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<'a, T>>
    where
        T: Resource + Send,
    {
        unsafe { self.resources.borrow_mut::<T>() }
    }
}
