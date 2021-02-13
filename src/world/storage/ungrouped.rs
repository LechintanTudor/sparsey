use crate::storage::{AbstractSparseSet, SparseSet};
use crate::world::Component;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct UngroupedComponents {
    sparse_sets: HashMap<TypeId, AtomicRefCell<Box<dyn AbstractSparseSet>>>,
}

impl UngroupedComponents {
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.sparse_sets
            .entry(TypeId::of::<T>())
            .or_insert_with(|| AtomicRefCell::new(Box::new(SparseSet::<T>::default())));
    }

    pub fn clear(&mut self) {
        for sparse_set in self.sparse_sets.values_mut() {
            sparse_set.get_mut().clear();
        }
    }

    pub fn borrow_abstract(&self, component: &TypeId) -> Option<AtomicRef<dyn AbstractSparseSet>> {
        self.sparse_sets
            .get(component)
            .map(|s| AtomicRef::map(s.borrow(), |s| Box::as_ref(s)))
    }

    pub fn borrow_abstract_mut(
        &self,
        component: &TypeId,
    ) -> Option<AtomicRefMut<dyn AbstractSparseSet>> {
        self.sparse_sets
            .get(component)
            .map(|s| AtomicRefMut::map(s.borrow_mut(), |s| Box::as_mut(s)))
    }

    pub fn iter_sparse_sets_mut(&mut self) -> impl Iterator<Item = &mut dyn AbstractSparseSet> {
        self.sparse_sets
            .values_mut()
            .map(|sparse_set| Box::as_mut(sparse_set.get_mut()))
    }
}
