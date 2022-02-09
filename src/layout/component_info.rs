use crate::storage::{Component, ComponentStorage};
use std::any::TypeId;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Holds information about a `Component` type.
pub struct ComponentInfo(Box<dyn AbstractType>);

impl ComponentInfo {
    /// Creates a new `ComponentInfo` for the given `Component` type.
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self(Box::new(Type::<T>(PhantomData)))
    }

    /// Returns the `TypeId` of the `Component`.
    pub fn type_id(&self) -> TypeId {
        self.0.type_id()
    }

    /// Returns an empty `ComponentStorage` for the `Component`.
    pub(crate) fn create_storage(&self) -> ComponentStorage {
        self.0.create_storage()
    }
}

impl Clone for ComponentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for ComponentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.type_id().eq(&other.type_id())
    }
}

impl Eq for ComponentInfo {}

impl PartialOrd for ComponentInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.type_id().partial_cmp(&other.type_id())
    }
}

impl Ord for ComponentInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_id().cmp(&other.type_id())
    }
}

impl Hash for ComponentInfo {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.type_id().hash(state);
    }
}

impl fmt::Debug for ComponentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ComponentInfo").field(&self.0.type_id()).finish()
    }
}

#[derive(Copy, Clone)]
struct Type<T>(PhantomData<*const T>);

unsafe impl<T> Send for Type<T> {}
unsafe impl<T> Sync for Type<T> {}

impl<T> Default for Type<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe trait AbstractType: Send + Sync + 'static {
    fn type_id(&self) -> TypeId;

    fn create_storage(&self) -> ComponentStorage;

    fn clone(&self) -> Box<dyn AbstractType>;
}

unsafe impl<T> AbstractType for Type<T>
where
    T: Component,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn create_storage(&self) -> ComponentStorage {
        ComponentStorage::new::<T>()
    }

    fn clone(&self) -> Box<dyn AbstractType> {
        Box::new(Type::<T>::default())
    }
}
