//! Manages component storages and handles component grouping.

mod component_set;
mod group;
mod group_info;
mod masks;

pub use self::component_set::*;
pub(crate) use self::group::*;
pub use self::group_info::*;
pub(crate) use self::masks::*;
use crate::layout::Layout;
pub use crate::storage::Component;
use crate::storage::{ComponentStorage, Entity};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::ops::Range;
use std::ptr::NonNull;
use std::slice;

/// Container for component storages. Also manages component grouping.
#[derive(Default)]
pub struct ComponentStorages {
    storages: Vec<AtomicRefCell<ComponentStorage>>,
    component_info: FxHashMap<TypeId, ComponentData>,
    groups: Vec<Group>,
    family_ranges: Vec<Range<usize>>,
}

impl ComponentStorages {
    pub(crate) unsafe fn new(
        layout: &Layout,
        spare_storages: &mut FxHashMap<TypeId, ComponentStorage>,
    ) -> Self {
        let mut component_info = FxHashMap::default();
        let mut storages = Vec::new();
        let mut groups = Vec::new();
        let mut family_ranges = Vec::new();

        // Iterate group families
        for (family_index, family) in layout.families().iter().enumerate() {
            let family_group_index = groups.len();
            let family_storage_index = storages.len();

            let mut prev_group_arity = 0_usize;

            // Iterate groups in a group family
            for (group_offset, &group_arity) in family.group_arities().iter().enumerate() {
                let new_storage_index = storages.len();
                let new_components = &family.components()[prev_group_arity..group_arity];

                // Iterate new components in a group
                for (component_offset, component) in new_components.iter().enumerate() {
                    component_info.insert(
                        component.type_id(),
                        ComponentData {
                            storage_index: storages.len(),
                            family_mask: FamilyMask::from_family_index(family_index),
                            group_mask: GroupMask::new(groups.len(), group_arity, family.arity()),
                            group_info: Some(StorageGroupInfo {
                                family_group_index: family_group_index as u32,
                                group_offset: group_offset as u32,
                                storage_mask: StorageMask::from_storage_index(
                                    prev_group_arity + component_offset,
                                ),
                            }),
                        },
                    );

                    let storage = spare_storages
                        .remove(&component.type_id())
                        .unwrap_or_else(|| component.create_storage());

                    storages.push(AtomicRefCell::new(storage));
                }

                groups.push(Group::new(
                    family_storage_index,
                    new_storage_index,
                    storages.len(),
                ));
                prev_group_arity = group_arity;
            }

            family_ranges.push(family_group_index..groups.len());
        }

        for (type_id, storage) in spare_storages.drain() {
            component_info.insert(
                type_id,
                ComponentData {
                    storage_index: storages.len(),
                    family_mask: FamilyMask::NONE,
                    group_mask: GroupMask::NONE,
                    group_info: None,
                },
            );

            storages.push(AtomicRefCell::new(storage));
        }

        Self {
            component_info,
            storages,
            family_ranges,
            groups,
        }
    }

    pub(crate) fn into_storages(mut self) -> FxHashMap<TypeId, ComponentStorage> {
        let mut storages = FxHashMap::default();

        for (type_id, info) in self.component_info {
            let storage = std::mem::replace(
                self.storages[info.storage_index].get_mut(),
                ComponentStorage::new::<()>(),
            );
            storages.insert(type_id, storage);
        }

        storages
    }

    pub(crate) fn register<T>(&mut self)
    where
        T: Component,
    {
        unsafe {
            self.register_with(TypeId::of::<T>(), ComponentStorage::new::<T>);
        }
    }

    pub(crate) unsafe fn register_with(
        &mut self,
        type_id: TypeId,
        storage_builder: impl FnOnce() -> ComponentStorage,
    ) {
        if let Entry::Vacant(entry) = self.component_info.entry(type_id) {
            entry.insert(ComponentData {
                storage_index: self.storages.len(),
                family_mask: FamilyMask::default(),
                group_mask: GroupMask::default(),
                group_info: None,
            });

            self.storages.push(AtomicRefCell::new(storage_builder()));
        }
    }

    #[must_use]
    pub(crate) fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.is_type_id_registered(TypeId::of::<T>())
    }

    #[inline]
    #[must_use]
    pub(crate) fn is_type_id_registered(&self, component_type_id: TypeId) -> bool {
        self.component_info.contains_key(&component_type_id)
    }

    pub(crate) unsafe fn group_families(
        &mut self,
        family_mask: FamilyMask,
        entities: impl Iterator<Item = Entity> + Clone,
    ) {
        for family_index in family_mask.iter_indexes() {
            let family_range = self.family_ranges.get_unchecked(family_index);

            entities.clone().for_each(|entity| {
                group_family(
                    &mut self.storages,
                    &mut self.groups,
                    family_range.clone(),
                    entity,
                );
            });
        }
    }

    pub(crate) unsafe fn ungroup_families(
        &mut self,
        family_mask: FamilyMask,
        group_mask: GroupMask,
        entities: impl Iterator<Item = Entity> + Clone,
    ) {
        for family_index in family_mask.iter_indexes() {
            let family_range = self.family_ranges.get_unchecked(family_index);

            entities.clone().for_each(|entity| {
                ungroup_family(
                    &mut self.storages,
                    &mut self.groups,
                    family_range.clone(),
                    group_mask,
                    entity,
                );
            });
        }
    }

    pub(crate) fn group_all_families(&mut self, entities: impl Iterator<Item = Entity> + Clone) {
        for family_range in self.family_ranges.iter() {
            entities.clone().for_each(|entity| unsafe {
                group_family(
                    &mut self.storages,
                    &mut self.groups,
                    family_range.clone(),
                    entity,
                );
            });
        }
    }

    pub(crate) fn ungroup_all_families(&mut self, entities: impl Iterator<Item = Entity> + Clone) {
        for family_range in self.family_ranges.iter() {
            entities.clone().for_each(|entity| unsafe {
                ungroup_family(
                    &mut self.storages,
                    &mut self.groups,
                    family_range.clone(),
                    GroupMask::ALL,
                    entity,
                );
            });
        }
    }

    /// Removes all entities and components from the storages.
    pub(crate) fn clear(&mut self) {
        for storage in self.storages.iter_mut() {
            storage.get_mut().clear();
        }

        for group in self.groups.iter_mut() {
            group.clear();
        }
    }

    pub(crate) fn borrow_with_info(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRef<ComponentStorage>, Option<GroupInfo>)> {
        let info = self.component_info.get(type_id)?;

        unsafe {
            Some((
                self.storages.get_unchecked(info.storage_index).borrow(),
                info.group_info.map(|info| {
                    let groups = slice::from_raw_parts(
                        self.groups.as_ptr().add(info.family_group_index as usize),
                        info.group_offset as usize + 1,
                    );
                    GroupInfo::new(groups, info.storage_mask)
                }),
            ))
        }
    }

    pub(crate) fn borrow_with_info_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, Option<GroupInfo>)> {
        let info = self.component_info.get(type_id)?;

        unsafe {
            Some((
                self.storages.get_unchecked(info.storage_index).borrow_mut(),
                info.group_info.map(|info| {
                    let groups = slice::from_raw_parts(
                        self.groups.as_ptr().add(info.family_group_index as usize),
                        info.group_offset as usize + 1,
                    );
                    GroupInfo::new(groups, info.storage_mask)
                }),
            ))
        }
    }

    pub(crate) fn get_as_ptr_with_family_mask(
        &self,
        type_id: &TypeId,
    ) -> Option<(NonNull<ComponentStorage>, FamilyMask)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                NonNull::new_unchecked(self.storages.get_unchecked(info.storage_index).as_ptr()),
                info.family_mask,
            )
        })
    }

    pub(crate) fn get_as_ptr_with_masks(
        &self,
        type_id: &TypeId,
    ) -> Option<(NonNull<ComponentStorage>, FamilyMask, GroupMask)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                NonNull::new_unchecked(self.storages.get_unchecked(info.storage_index).as_ptr()),
                info.family_mask,
                info.group_mask,
            )
        })
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
        self.storages.iter_mut().map(AtomicRefCell::get_mut)
    }
}

#[derive(Clone, Copy)]
struct ComponentData {
    /// Index of the storage in `storages`.
    storage_index: usize,
    /// Bitmask for the family index to which the storage belongs.
    family_mask: FamilyMask,
    /// Bitmask for the group indexes to which the storage belongs.
    group_mask: GroupMask,
    /// Group info for the storage, if grouped.
    group_info: Option<StorageGroupInfo>,
}

#[derive(Clone, Copy)]
struct StorageGroupInfo {
    /// Index in `storages` at which the family starts.
    family_group_index: u32,
    /// Offset from the family group index.
    group_offset: u32,
    /// Bitmask for the index in the family of the storage.
    storage_mask: StorageMask,
}
