use crate::components::{Component, GroupedComponentStorages, UngroupedComponentStorages};
use crate::group::GroupInfo;
use crate::layout::Layout;
use crate::storage::{ComponentStorage, Entity};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;

/// Container for grouped and ungrouped component storages.
#[derive(Default)]
pub struct ComponentStorages {
    pub(crate) grouped: GroupedComponentStorages,
    pub(crate) ungrouped: UngroupedComponentStorages,
}

impl ComponentStorages {
    pub(crate) fn clear(&mut self) {
        self.grouped.clear();
        self.ungrouped.clear();
    }

    pub(crate) fn register<T>(&mut self)
    where
        T: Component,
    {
        if !self.grouped.contains(&TypeId::of::<T>()) {
            self.ungrouped.register::<T>();
        }
    }

    pub(crate) unsafe fn register_storage(&mut self, component: TypeId, storage: ComponentStorage) {
        if !self.grouped.contains(&component) {
            self.ungrouped.register_storage(component, storage);
        }
    }

    pub(crate) fn is_registered(&self, component: &TypeId) -> bool {
        self.grouped.contains(component) || self.ungrouped.contains(component)
    }

    pub(crate) fn set_layout(&mut self, layout: &Layout, entities: &[Entity]) {
        let mut storages = FxHashMap::<TypeId, ComponentStorage>::default();
        self.grouped.drain_into(&mut storages);
        self.ungrouped.drain_into(&mut storages);

        self.grouped = unsafe { GroupedComponentStorages::with_layout(layout, &mut storages) };
        self.ungrouped = UngroupedComponentStorages::from_storages(&mut storages);

        for i in 0..self.grouped.group_family_count() {
            for &entity in entities {
                unsafe {
                    self.grouped.group_components(i, entity);
                }
            }
        }
    }

    pub(crate) fn get_group_family(&self, component: &TypeId) -> Option<usize> {
        self.grouped.get_group_family(component)
    }

    #[allow(dead_code)]
    pub(crate) fn borrow(&self, component: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
        match self.grouped.borrow(component) {
            storage @ Some(_) => storage,
            None => self.ungrouped.borrow(component),
        }
    }

    pub(crate) fn borrow_mut(&self, component: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
        match self.grouped.borrow_mut(component) {
            storage @ Some(_) => storage,
            None => self.ungrouped.borrow_mut(component),
        }
    }

    pub(crate) fn borrow_with_info(
        &self,
        component: &TypeId,
    ) -> Option<(AtomicRef<ComponentStorage>, Option<GroupInfo>)> {
        match self.grouped.borrow_with_info(component) {
            Some((storage, info)) => Some((storage, Some(info))),
            None => self
                .ungrouped
                .borrow(component)
                .map(|storage| (storage, None)),
        }
    }

    pub(crate) fn borrow_with_info_mut(
        &self,
        component: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, Option<GroupInfo>)> {
        match self.grouped.borrow_with_info_mut(component) {
            Some((storage, info)) => Some((storage, Some(info))),
            None => self
                .ungrouped
                .borrow_mut(component)
                .map(|storage| (storage, None)),
        }
    }

    pub(crate) fn borrow_with_familiy_mut(
        &self,
        component: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, Option<usize>)> {
        match self.grouped.borrow_with_familiy_mut(component) {
            Some((storage, index)) => Some((storage, Some(index))),
            None => self
                .ungrouped
                .borrow_mut(component)
                .map(|storage| (storage, None)),
        }
    }

    pub(crate) fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> + '_ {
        self.grouped
            .iter_storages_mut()
            .chain(self.ungrouped.iter_storages_mut())
    }
}
