use crate::registry::Component;
use crate::storage::{AbstractSparseSet, Entity, SparseSet};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

#[derive(Default)]
pub struct Components {
    sets: HashMap<TypeId, AtomicRefCell<Box<dyn AbstractSparseSet>>>,
}

impl Components {
    pub fn destroy(&mut self, entity: Entity) {
        for set in self.sets.values_mut() {
            set.get_mut().delete(entity);
        }
    }

    pub fn maintain(&mut self) {
        self.sets.values_mut().for_each(|s| s.get_mut().maintain());
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.sets
            .entry(TypeId::of::<T>())
            .or_insert_with(|| AtomicRefCell::new(Box::new(SparseSet::<T>::default())));
    }

    pub fn borrow<T>(&self) -> Option<AtomicRef<SparseSet<T>>>
    where
        T: Component,
    {
        self.borrow_abstract(TypeId::of::<T>()).map(|s| {
            AtomicRef::map(s, |s| match s.as_any().downcast_ref::<SparseSet<T>>() {
                Some(s) => s,
                None => unsafe { unreachable_unchecked() },
            })
        })
    }

    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.borrow_abstract_mut(TypeId::of::<T>()).map(|s| {
            AtomicRefMut::map(s, |s| match s.as_mut_any().downcast_mut::<SparseSet<T>>() {
                Some(s) => s,
                None => unsafe { unreachable_unchecked() },
            })
        })
    }

    pub fn borrow_abstract(&self, component: TypeId) -> Option<AtomicRef<dyn AbstractSparseSet>> {
        self.sets
            .get(&component)
            .map(|s| AtomicRef::map(s.borrow(), |s| s.as_ref()))
    }

    pub fn borrow_abstract_mut(
        &self,
        component: TypeId,
    ) -> Option<AtomicRefMut<dyn AbstractSparseSet>> {
        self.sets
            .get(&component)
            .map(|s| AtomicRefMut::map(s.borrow_mut(), |s| s.as_mut()))
    }
}
