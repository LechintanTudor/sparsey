use crate::resources::Resource;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub(crate) struct ResourceStorage {
    resources: FxHashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

impl ResourceStorage {
    pub fn insert<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .insert(TypeId::of::<T>(), AtomicRefCell::new(Box::new(resource)))
            .map(|c| unsafe { *c.into_inner().downcast().unwrap_unchecked() })
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(|c| unsafe { *c.into_inner().downcast().unwrap_unchecked() })
    }

    pub fn delete(&mut self, resource_type_id: &TypeId) -> bool {
        self.resources.remove(resource_type_id).is_some()
    }

    pub fn contains(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains_key(resource_type_id)
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    #[inline]
    pub fn borrow<T>(&self) -> Option<AtomicRef<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|c| {
            AtomicRef::map(c.borrow(), |c| unsafe { c.deref().downcast_ref().unwrap_unchecked() })
        })
    }

    #[inline]
    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|c| {
            AtomicRefMut::map(c.borrow_mut(), |c| unsafe {
                c.deref_mut().downcast_mut().unwrap_unchecked()
            })
        })
    }
}
