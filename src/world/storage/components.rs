use crate::data::{AtomicRef, AtomicRefMut, Component, Entity, TypeErasedSparseSet};
use crate::query::{Comp, CompMut, SparseSetRefMutBorrow};
use crate::world::{GroupedComponents, Layout, UngroupedComponents};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct Components {
    pub(crate) grouped: GroupedComponents,
    pub(crate) ungrouped: UngroupedComponents,
}

impl Components {
    pub fn clear(&mut self) {
        self.grouped.clear();
        self.ungrouped.clear();
    }

    pub fn clear_flags(&mut self) {
        self.iter_sparse_sets_mut()
            .for_each(|sparse_set| sparse_set.clear_flags())
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        if !self.grouped.contains(&TypeId::of::<T>()) {
            self.ungrouped.register::<T>();
        }
    }

    pub fn register_storage(&mut self, sparse_set: TypeErasedSparseSet) {
        if !self.grouped.contains(&sparse_set.component_type_id()) {
            self.ungrouped.register_storage(sparse_set);
        }
    }

    pub fn set_layout(&mut self, layout: &Layout, entities: &[Entity]) {
        let mut sparse_sets = HashMap::<TypeId, TypeErasedSparseSet>::new();

        for sparse_set in self.grouped.drain().chain(self.ungrouped.drain()) {
            sparse_sets.insert(sparse_set.component_type_id(), sparse_set);
        }

        self.grouped = GroupedComponents::with_layout(&layout, &mut sparse_sets);
        self.ungrouped = UngroupedComponents::from_sparse_sets(&mut sparse_sets);

        for i in 0..self.grouped.group_count() {
            for &entity in entities {
                self.grouped.group_components(i, entity);
            }
        }
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        match self.grouped.borrow(&TypeId::of::<T>()) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                    self.grouped.get_subgroup_info(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped.borrow(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(Comp::new(
                        AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        match self.grouped.borrow_mut(&TypeId::of::<T>()) {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                    self.grouped.get_subgroup_info(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped.borrow_mut(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(CompMut::new(
                        AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    pub fn borrow_sparse_set_mut<T>(&self) -> Option<SparseSetRefMutBorrow<T>>
    where
        T: Component,
    {
        match self.ungrouped.borrow_mut(&TypeId::of::<T>()) {
            Some(sparse_set) => Some(SparseSetRefMutBorrow::new(AtomicRefMut::map_into(
                sparse_set,
                |sparse_set| sparse_set.to_ref_mut::<T>(),
            ))),
            None => {
                let sparse_set = self.grouped.borrow_mut(&TypeId::of::<T>())?;
                Some(SparseSetRefMutBorrow::new(AtomicRefMut::map_into(
                    sparse_set,
                    |sparse_set| sparse_set.to_ref_mut::<T>(),
                )))
            }
        }
    }

    pub fn iter_sparse_sets_mut(&mut self) -> impl Iterator<Item = &mut TypeErasedSparseSet> + '_ {
        self.grouped
            .iter_sparse_sets_mut()
            .chain(self.ungrouped.iter_sparse_sets_mut())
    }
}
