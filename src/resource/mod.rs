mod borrow;
mod resource;

pub use self::borrow::*;
pub use self::resource::*;

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::{any, fmt, mem};

#[derive(Default)]
pub struct ResourceStorage {
    resources: FxHashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

impl ResourceStorage {
    pub fn insert<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        match self.resources.entry(TypeId::of::<T>()) {
            Entry::Vacant(entry) => {
                entry.insert(AtomicRefCell::new(Box::new(resource)));
                None
            }
            Entry::Occupied(mut entry) => {
                let old_resource = unsafe {
                    entry
                        .get_mut()
                        .get_mut()
                        .as_mut()
                        .downcast_mut::<T>()
                        .unwrap_unchecked()
                };

                Some(mem::replace(old_resource, resource))
            }
        }
    }

    pub fn remove<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(|cell| unsafe { *cell.into_inner().downcast().unwrap_unchecked() })
    }

    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: Resource,
    {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    #[must_use]
    pub fn get_mut<T>(&mut self) -> &mut T
    where
        T: Resource,
    {
        self.try_get_mut()
            .unwrap_or_else(|| panic_missing_res::<T>())
    }

    #[must_use]
    pub fn borrow<T>(&self) -> Res<T>
    where
        T: Resource,
    {
        self.try_borrow()
            .unwrap_or_else(|| panic_missing_res::<T>())
    }

    #[must_use]
    pub fn borrow_mut<T>(&self) -> ResMut<T>
    where
        T: Resource,
    {
        self.try_borrow_mut()
            .unwrap_or_else(|| panic_missing_res::<T>())
    }

    #[must_use]
    pub fn try_get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Resource,
    {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .map(|cell| unsafe { cell.get_mut().downcast_mut().unwrap_unchecked() })
    }

    #[must_use]
    pub fn try_borrow<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|cell| {
            Res(AtomicRef::map(cell.borrow(), |cell| unsafe {
                cell.downcast_ref().unwrap_unchecked()
            }))
        })
    }

    #[must_use]
    pub fn try_borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        self.resources.get(&TypeId::of::<T>()).map(|cell| {
            ResMut(AtomicRefMut::map(cell.borrow_mut(), |cell| unsafe {
                cell.downcast_mut().unwrap_unchecked()
            }))
        })
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl fmt::Debug for ResourceStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ResourceStorage))
            .field("type_ids", &self.resources.keys())
            .finish_non_exhaustive()
    }
}

#[cold]
#[inline(never)]
fn panic_missing_res<T>() -> !
where
    T: Resource,
{
    panic!(
        "Tried to access missing resource of type '{}'",
        any::type_name::<T>(),
    );
}
