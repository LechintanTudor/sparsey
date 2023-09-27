use crate::resources::{Res, ResMut, Resource};
use crate::utils;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::fmt;

/// Unsafe resource storage. Unsafe because it can store `!Send` and `!Sync` resources while being
/// `Send + Sync` itself.
#[derive(Default)]
pub(crate) struct UnsafeResources {
    resources: FxHashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
    pub unsafe fn insert<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .insert(TypeId::of::<T>(), AtomicRefCell::new(Box::new(resource)))
            .map(|c| *c.into_inner().downcast().unwrap_unchecked())
    }

    pub unsafe fn borrow<T>(&self) -> Res<T>
    where
        T: Resource,
    {
        self.try_borrow()
            .unwrap_or_else(|| utils::panic_missing_res::<T>())
    }

    pub unsafe fn borrow_mut<T>(&self) -> ResMut<T>
    where
        T: Resource,
    {
        self.try_borrow_mut()
            .unwrap_or_else(|| utils::panic_missing_res::<T>())
    }

    pub unsafe fn get_mut<T>(&mut self) -> &mut T
    where
        T: Resource,
    {
        self.try_get_mut()
            .unwrap_or_else(|| utils::panic_missing_res::<T>())
    }

    pub unsafe fn try_borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|c| {
            Res::new(AtomicRef::map(c.borrow(), |c| {
                c.downcast_ref().unwrap_unchecked()
            }))
        })
    }

    pub unsafe fn try_borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|c| {
            ResMut::new(AtomicRefMut::map(c.borrow_mut(), |c| {
                c.downcast_mut().unwrap_unchecked()
            }))
        })
    }

    pub unsafe fn try_get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Resource,
    {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .map(|c| c.get_mut().downcast_mut().unwrap_unchecked())
    }

    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.contains_type_id(TypeId::of::<T>())
    }

    #[inline]
    #[must_use]
    pub fn contains_type_id(&self, resource_type_id: TypeId) -> bool {
        self.resources.contains_key(&resource_type_id)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub unsafe fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(|c| *c.into_inner().downcast().unwrap_unchecked())
    }

    #[inline]
    pub unsafe fn delete(&mut self, resource_type_id: TypeId) -> bool {
        self.resources.remove(&resource_type_id).is_some()
    }

    #[inline]
    pub unsafe fn clear(&mut self) {
        self.resources.clear()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DebugResourceTypeIdSet<'a>(pub &'a UnsafeResources);

impl fmt::Debug for DebugResourceTypeIdSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.0.resources.keys()).finish()
    }
}

impl fmt::Debug for UnsafeResources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnsafeResources")
            .field("resource_type_ids", &DebugResourceTypeIdSet(self))
            .finish_non_exhaustive()
    }
}
