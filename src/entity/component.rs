use crate::entity::ComponentSparseSet;
use std::any::{self, TypeId};
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait Component: Send + Sync + 'static {
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}

#[derive(Clone, Copy)]
pub struct ComponentData(&'static dyn AbstractComponentData);

impl ComponentData {
    #[must_use]
    pub const fn new<T>() -> Self
    where
        T: Component,
    {
        Self(&ComponentDataImpl::<T>(PhantomData))
    }

    #[inline]
    #[must_use]
    pub fn type_id(&self) -> TypeId {
        self.0.type_id()
    }

    #[inline]
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        self.0.type_name()
    }

    #[inline]
    #[must_use]
    pub fn create_sparse_set(&self) -> ComponentSparseSet {
        self.0.create_sparse_set()
    }
}

impl PartialEq for ComponentData {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.type_id().eq(&other.type_id())
    }
}

impl Eq for ComponentData {
    // Empty
}

impl Hash for ComponentData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.type_id().hash(state);
    }
}

impl fmt::Debug for ComponentData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ComponentData))
            .field("type_id", &self.0.type_id())
            .field("type_name", &self.0.type_name())
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
