use crate::component::ComponentSparseSet;
use std::any::{self, TypeId};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Holds information about a type.
#[derive(Clone, Copy)]
pub struct TypeData(&'static dyn AbstractTypeData);

impl TypeData {
    /// Returns the type data for type `T`.
    #[must_use]
    pub const fn new<T>() -> Self
    where
        T: Send + Sync + 'static,
    {
        Self(&ComponentDataImpl::<T>(PhantomData))
    }

    /// Returns the type id of the type used in [`new`](Self::new).
    #[inline]
    #[must_use]
    pub fn type_id(&self) -> TypeId {
        self.0.type_id()
    }

    /// Returns the type name of the type used in [`new`](Self::new).
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

impl PartialEq for TypeData {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.type_id().eq(&other.type_id())
    }
}

impl Eq for TypeData {
    // Empty
}

impl PartialOrd for TypeData {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeData {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_id().cmp(&other.0.type_id())
    }
}

impl Hash for TypeData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.type_id().hash(state);
    }
}

impl fmt::Debug for TypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ComponentData))
            .field("type_id", &self.type_id())
            .field("type_name", &self.type_name())
            .finish()
    }
}

unsafe trait AbstractTypeData: Send + Sync + 'static {
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

unsafe impl<T> AbstractTypeData for ComponentDataImpl<T>
where
    T: Send + Sync + 'static,
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
