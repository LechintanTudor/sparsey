use crate::storage::{AbstractSparseSet, SparseSet};
use crate::world::Component;
use std::any::{self, TypeId};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::marker::PhantomData;

pub struct LayoutComponent {
    component: Box<dyn AbstractLayoutComponent>,
}

impl LayoutComponent {
    pub fn new<C>() -> Self
    where
        C: Component,
    {
        Self {
            component: Box::new(GenericLayoutComponent::<C>::default()),
        }
    }

    pub fn component_type_id(&self) -> TypeId {
        self.component.component_type_id()
    }

    pub fn component_type_name(&self) -> &'static str {
        self.component.component_type_name()
    }

    pub fn create_sparse_set(&self) -> Box<dyn AbstractSparseSet> {
        self.component.create_sparse_set()
    }
}

impl PartialEq for LayoutComponent {
    fn eq(&self, other: &Self) -> bool {
        self.component_type_id().eq(&other.component_type_id())
    }
}

impl Eq for LayoutComponent {}

impl PartialOrd for LayoutComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.component_type_id()
            .partial_cmp(&other.component_type_id())
    }
}

impl Ord for LayoutComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.component_type_id().cmp(&other.component_type_id())
    }
}

trait AbstractLayoutComponent
where
    Self: Send + Sync + 'static,
{
    fn component_type_id(&self) -> TypeId;

    fn component_type_name(&self) -> &'static str;

    fn create_sparse_set(&self) -> Box<dyn AbstractSparseSet>;
}

#[derive(Copy, Clone)]
struct GenericLayoutComponent<C> {
    _phantom: PhantomData<*const C>,
}

unsafe impl<C> Send for GenericLayoutComponent<C> {}
unsafe impl<C> Sync for GenericLayoutComponent<C> {}

impl<C> Default for GenericLayoutComponent<C>
where
    C: Component,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<C> AbstractLayoutComponent for GenericLayoutComponent<C>
where
    C: Component,
{
    fn component_type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn component_type_name(&self) -> &'static str {
        any::type_name::<C>()
    }

    fn create_sparse_set(&self) -> Box<dyn AbstractSparseSet> {
        Box::new(SparseSet::<C>::default())
    }
}
