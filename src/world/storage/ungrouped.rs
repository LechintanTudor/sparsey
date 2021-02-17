use crate::data::{AtomicRef, AtomicRefCell, AtomicRefMut, Component, TypeErasedSparseSet};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct UngroupedComponents {
    sparse_sets: HashMap<TypeId, AtomicRefCell<TypeErasedSparseSet>>,
}

impl UngroupedComponents {
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.sparse_sets
            .entry(TypeId::of::<T>())
            .or_insert_with(|| AtomicRefCell::new(TypeErasedSparseSet::new::<T>()));
    }

    pub fn clear(&mut self) {
        for sparse_set in self.sparse_sets.values_mut() {
            sparse_set.get_mut().clear();
        }
    }

    pub fn borrow(&self, component: &TypeId) -> Option<AtomicRef<TypeErasedSparseSet>> {
        self.sparse_sets
            .get(component)
            .map(|sparse_set| sparse_set.borrow())
    }

    pub fn borrow_mut(&self, component: &TypeId) -> Option<AtomicRefMut<TypeErasedSparseSet>> {
        self.sparse_sets
            .get(component)
            .map(|sparse_set| sparse_set.borrow_mut())
    }

    pub fn iter_sparse_sets_mut(&mut self) -> impl Iterator<Item = &mut TypeErasedSparseSet> {
        self.sparse_sets
            .values_mut()
            .map(|sparse_set| sparse_set.get_mut())
    }
}
