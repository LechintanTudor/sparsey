use crate::components::Component;
use crate::storage::ComponentStorage;
use std::any;
use std::any::TypeId;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Holds information about a `Component` type.
pub struct ComponentInfo {
    component: Box<dyn AbstractType>,
}

impl Clone for ComponentInfo {
    fn clone(&self) -> Self {
        Self {
            component: self.component.clone(),
        }
    }
}

impl ComponentInfo {
    /// Creates a new `ComponentInfo` for the given `Component` type.
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            component: Box::new(Type::<T>(PhantomData)),
        }
    }

    /// Returns the `TypeId` of the `Component`.
    pub fn type_id(&self) -> TypeId {
        self.component.type_id()
    }

    /// Returns the type name of the `Component`.
    pub fn type_name(&self) -> &'static str {
        self.component.type_name()
    }

    /// Returns an empty `ComponentStorage` for the `Component`.
    pub fn create_storage(&self) -> ComponentStorage {
        self.component.create_storage()
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

#[derive(Copy, Clone)]
struct Type<T>(PhantomData<*const T>);

unsafe impl<T> Send for Type<T> {}
unsafe impl<T> Sync for Type<T> {}

impl<T> Default for Type<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe trait AbstractType
where
    Self: Send + Sync + 'static,
{
    fn type_id(&self) -> TypeId;

    fn type_name(&self) -> &'static str;

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

    fn type_name(&self) -> &'static str {
        any::type_name::<T>()
    }

    fn create_storage(&self) -> ComponentStorage {
        ComponentStorage::new::<T>()
    }

    fn clone(&self) -> Box<dyn AbstractType> {
        Box::new(Type::<T>::default())
    }
}
