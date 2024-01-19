use crate::entity::ComponentSparseSet;
use std::any::{self, TypeId};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Marker trait for components that can be added to entities.
pub trait Component: Send + Sync + 'static {
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}

/// Holds information about a component type.
#[derive(Clone, Copy)]
pub struct ComponentData(&'static dyn AbstractComponentData);

impl ComponentData {
    /// Returns the component data for components of type `T`.
    #[must_use]
    pub const fn new<T>() -> Self
    where
        T: Component,
    {
        Self(&ComponentDataImpl::<T>(PhantomData))
    }

    /// Returns the type id of the component type used in [`new`](Self::new).
    #[inline]
    #[must_use]
    pub fn type_id(&self) -> TypeId {
        self.0.type_id()
    }

    /// Returns the type name of the component type used in [`new`](Self::new).
    #[inline]
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        self.0.type_name()
    }

    #[inline]
    #[must_use]
    pub(crate) fn create_sparse_set(&self) -> ComponentSparseSet {
        self.0.create_sparse_set()
    }
}

impl PartialEq for ComponentData {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.type_id().eq(&other.type_id())
    }
}

impl Eq for ComponentData {
    // Empty
}

impl PartialOrd for ComponentData {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComponentData {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_id().cmp(&other.0.type_id())
    }
}

impl Hash for ComponentData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.type_id().hash(state);
    }
}

impl fmt::Debug for ComponentData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ComponentData))
            .field("type_id", &self.type_id())
            .field("type_name", &self.type_name())
            .finish()
    }
}

unsafe trait AbstractComponentData: Send + Sync + 'static {
    #[must_use]
    fn type_id(&self) -> TypeId;

    #[must_use]
    fn type_name(&self) -> &'static str;

    #[must_use]
    fn create_sparse_set(&self) -> ComponentSparseSet;
}

struct ComponentDataImpl<T>(PhantomData<*const T>);

unsafe impl<T> Send for ComponentDataImpl<T> {
    // Empty
}

unsafe impl<T> Sync for ComponentDataImpl<T> {
    // Empty
}

unsafe impl<T> AbstractComponentData for ComponentDataImpl<T>
where
    T: Component,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> &'static str {
        any::type_name::<T>()
    }

    fn create_sparse_set(&self) -> ComponentSparseSet {
        ComponentSparseSet::new::<T>()
    }
}
