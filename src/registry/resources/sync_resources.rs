use crate::registry::{Res, ResMut, Resource, UnsafeResources};

#[derive(Copy, Clone)]
pub struct SyncResources<'a> {
    internal: &'a UnsafeResources,
}

impl<'a> SyncResources<'a> {
    pub(crate) fn new(internal: &'a UnsafeResources) -> Self {
        Self { internal }
    }

    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource + Sync,
    {
        unsafe { self.internal.borrow::<T>() }
    }

    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource + Send,
    {
        unsafe { self.internal.borrow_mut::<T>() }
    }
}
