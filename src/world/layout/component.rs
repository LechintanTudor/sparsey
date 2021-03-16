use crate::data::{Component, TypeErasedSparseSet};
use std::any;
use std::any::TypeId;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Holds information about a component type.
pub struct LayoutComponent {
    component: Box<dyn AbstractType>,
}

unsafe impl Send for LayoutComponent {}
unsafe impl Sync for LayoutComponent {}

impl Clone for LayoutComponent {
    fn clone(&self) -> Self {
        Self {
            component: self.component.clone(),
        }
    }
}

impl LayoutComponent {
    /// Create a new `LayoutComponent` for the given type.
    pub fn new<C>() -> Self
    where
        C: Component,
    {
        Self {
            component: Box::new(Type::<C>(PhantomData)),
        }
    }

    /// Get the `TypeId` of the component type described by the `LayoutComponent`.
    pub fn type_id(&self) -> TypeId {
        self.component.type_id()
    }

    /// Get the name of the component type described by the `LayoutComponent`.
    pub fn type_name(&self) -> &'static str {
        self.component.type_name()
    }

    /// Create a `TypeErasedSparseSet` for the component type described by the `LayoutComponent`.
    pub fn new_sparse_set(&self) -> TypeErasedSparseSet {
        self.component.new_sparse_set()
    }
}

impl PartialEq for LayoutComponent {
    fn eq(&self, other: &Self) -> bool {
        self.type_id().eq(&other.type_id())
    }
}

impl Eq for LayoutComponent {}

impl PartialOrd for LayoutComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.type_id().partial_cmp(&other.type_id())
    }
}

impl Ord for LayoutComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_id().cmp(&other.type_id())
    }
}

impl Hash for LayoutComponent {
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

unsafe trait AbstractType {
    fn type_id(&self) -> TypeId;

    fn type_name(&self) -> &'static str;

    fn new_sparse_set(&self) -> TypeErasedSparseSet;

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

    fn new_sparse_set(&self) -> TypeErasedSparseSet {
        TypeErasedSparseSet::new::<T>()
    }

    fn clone(&self) -> Box<dyn AbstractType> {
        Box::new(Type::<T>::default())
    }
}
