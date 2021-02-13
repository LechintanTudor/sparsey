use crate::storage::{AbstractSparseSet, SparseSet};
use crate::world::{
    Comp, CompMut, Component, GroupSetViewMut, GroupedComponents, SparseSetRefMut,
    UngroupedComponents, WorldLayout,
};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashSet;
use std::hint::unreachable_unchecked;

pub(crate) struct Components {
    grouped: GroupedComponents,
    ungrouped: UngroupedComponents,
}

impl Components {
    pub fn new(layout: &WorldLayout) -> Self {
        Self {
            grouped: GroupedComponents::new(layout),
            ungrouped: UngroupedComponents::default(),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        if !self.grouped.contains(&TypeId::of::<T>()) {
            self.ungrouped.register::<T>();
        }
    }

    pub fn clear(&mut self) {
        self.grouped.clear();
        self.ungrouped.clear();
    }

    pub unsafe fn iter_sparse_sets_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut dyn AbstractSparseSet> + '_ {
        self.grouped
            .iter_sparse_sets_mut()
            .chain(self.ungrouped.iter_sparse_sets_mut())
    }

    pub fn get_group_index(&self, type_id: &TypeId) -> Option<usize> {
        self.grouped.get_group_index(type_id)
    }

    pub unsafe fn get_group_set(&mut self, group_indexes: &HashSet<usize>) -> GroupSetViewMut {
        self.grouped.get_group_set(group_indexes)
    }

    pub unsafe fn get_full_group_set(&mut self) -> GroupSetViewMut {
        self.grouped.get_full_group_set()
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();

        match self.grouped.borrow_abstract(&type_id) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    downcast_sparse_set::<T>(sparse_set),
                    self.grouped.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped.borrow_abstract(&type_id)?;

                unsafe { Some(Comp::new(downcast_sparse_set::<T>(sparse_set), None)) }
            }
        }
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();

        match unsafe { self.grouped.borrow_abstract_mut(&type_id) } {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    downcast_sparse_set_mut::<T>(sparse_set),
                    self.grouped.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped.borrow_abstract_mut(&type_id)?;

                unsafe { Some(CompMut::new(downcast_sparse_set_mut::<T>(sparse_set), None)) }
            }
        }
    }

    pub unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<SparseSetRefMut<T>>
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();
        let sparse_set = self
            .grouped
            .borrow_abstract_mut(&type_id)
            .or_else(|| self.ungrouped.borrow_abstract_mut(&type_id))?;

        Some(SparseSetRefMut::new(downcast_sparse_set_mut::<T>(
            sparse_set,
        )))
    }
}

unsafe fn downcast_sparse_set<T>(
    sparse_set: AtomicRef<dyn AbstractSparseSet>,
) -> AtomicRef<SparseSet<T>>
where
    T: Component,
{
    AtomicRef::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_ref::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}

unsafe fn downcast_sparse_set_mut<T>(
    sparse_set: AtomicRefMut<dyn AbstractSparseSet>,
) -> AtomicRefMut<SparseSet<T>>
where
    T: Component,
{
    AtomicRefMut::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_mut::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}
