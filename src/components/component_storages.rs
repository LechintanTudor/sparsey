use crate::components;
use crate::components::{FamilyMask, GroupInfo, GroupMask, GroupStatus, QueryMask, StorageMask};
use crate::layout::Layout;
use crate::storage::{Component, ComponentStorage, Entity};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::mem;
use std::ops::Range;
use std::ptr::NonNull;

/// Container for `ComponentStorage`s. Also manages component grouping.
#[derive(Default)]
pub struct ComponentStorages {
    storages: Vec<AtomicRefCell<ComponentStorage>>,
    component_info: FxHashMap<TypeId, ComponentInfo>,
    group_info: Vec<StorageGroupInfo>,
    groups: Vec<Group>,
    families: Vec<Range<usize>>,
}

impl ComponentStorages {
    pub(crate) unsafe fn new(
        layout: &Layout,
        spare_storages: &mut FxHashMap<TypeId, ComponentStorage>,
    ) -> Self {
        let mut component_info = FxHashMap::default();
        let mut storages = Vec::new();
        let mut group_info = Vec::new();
        let mut groups = Vec::new();
        let mut families = Vec::new();

        // Iterate group families.
        for (family_index, family) in layout.families().iter().enumerate() {
            let first_group_index = groups.len();
            let first_storage_index = storages.len();

            let components = family.components();
            let family_arity = family.arity();

            let mut prev_arity = 0_usize;

            // Iterate groups in a group family.
            for (group_offset, &arity) in family.group_arities().iter().enumerate() {
                let new_storage_index = storages.len();

                // Iterate new components in a group.
                for (component_offset, component) in
                    (&components[prev_arity..arity]).iter().enumerate()
                {
                    let type_id = component.type_id();
                    let group_mask = components::new_group_mask(groups.len(), arity, family_arity);

                    component_info.insert(
                        type_id,
                        ComponentInfo {
                            storage_index: storages.len(),
                            group_info_index: group_info.len(),
                            group_mask,
                            family_mask: 1 << family_index,
                        },
                    );

                    group_info.push(StorageGroupInfo {
                        family_index,
                        group_offset,
                        storage_mask: 1 << (prev_arity + component_offset),
                    });

                    let storage = spare_storages
                        .remove(&type_id)
                        .unwrap_or_else(|| component.create_storage());
                    storages.push(AtomicRefCell::new(storage));
                }

                groups.push(Group {
                    begin: first_storage_index,
                    new_begin: new_storage_index,
                    end: storages.len(),
                    len: 0,
                });

                prev_arity = arity;
            }

            families.push(first_group_index..groups.len());
        }

        for (type_id, storage) in spare_storages.drain() {
            component_info.insert(
                type_id,
                ComponentInfo {
                    storage_index: storages.len(),
                    group_info_index: usize::MAX,
                    family_mask: 0,
                    group_mask: 0,
                },
            );

            storages.push(AtomicRefCell::new(storage));
        }

        Self { component_info, storages, group_info, families, groups }
    }

    pub(crate) fn into_storages(mut self) -> FxHashMap<TypeId, ComponentStorage> {
        let mut storages = FxHashMap::default();

        for (type_id, info) in self.component_info {
            let storage = mem::replace(
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
            entry.insert(ComponentInfo {
                storage_index: self.storages.len(),
                group_info_index: usize::MAX,
                group_mask: 0,
                family_mask: 0,
            });

            self.storages.push(AtomicRefCell::new(storage_builder()));
        }
    }

    pub(crate) fn is_registered(&self, type_id: &TypeId) -> bool {
        self.component_info.contains_key(type_id)
    }

    pub(crate) unsafe fn group_components<'a, E>(&mut self, family_index: usize, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
    {
        let groups =
            self.groups.get_unchecked_mut(self.families.get_unchecked(family_index).clone());
        let storages = &mut self.storages;

        entities.into_iter().for_each(|&entity| {
            for group in groups.iter_mut() {
                let status = components::get_group_status(
                    storages.get_unchecked_mut(group.new_storage_range()),
                    group.len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => (),
                    GroupStatus::Ungrouped => {
                        components::group_components(
                            storages.get_unchecked_mut(group.storage_range()),
                            &mut group.len,
                            entity,
                        );
                    }
                    GroupStatus::MissingComponents => break,
                }
            }
        });
    }

    pub(crate) unsafe fn ungroup_components<'a, E>(
        &mut self,
        family_index: usize,
        group_mask: GroupMask,
        entities: E,
    ) where
        E: IntoIterator<Item = &'a Entity>,
    {
        let groups =
            self.groups.get_unchecked_mut(self.families.get_unchecked(family_index).clone());
        let storages = &mut self.storages;

        entities.into_iter().for_each(|&entity| {
            let mut ungroup_start = 0_usize;
            let mut ungroup_len = 0_usize;

            for (i, group) in groups.iter().enumerate() {
                let status = components::get_group_status(
                    storages.get_unchecked_mut(group.new_storage_range()),
                    group.len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => {
                        if ungroup_len == 0 {
                            ungroup_start = i;
                        }

                        ungroup_len += 1;
                    }
                    GroupStatus::Ungrouped => break,
                    GroupStatus::MissingComponents => break,
                }
            }

            let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

            for i in ungroup_range.rev().take_while(|i| (group_mask & (1 << i)) != 0) {
                let group = groups.get_unchecked_mut(i);
                let storages = storages.get_unchecked_mut(group.storage_range());
                components::ungroup_components(storages, &mut group.len, entity);
            }
        });
    }

    pub(crate) fn group_all_components<'a, E>(&mut self, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        let entities = entities.into_iter();

        for i in 0..self.families.len() {
            unsafe {
                self.group_components(i, entities.clone());
            }
        }
    }

    pub(crate) fn ungroup_all_components<'a, E>(&mut self, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        let entities = entities.into_iter();

        for i in 0..self.families.len() {
            unsafe {
                self.ungroup_components(i, GroupMask::MAX, entities.clone());
            }
        }
    }

    /// Removes all entities and components from the storages.
    pub(crate) fn clear(&mut self) {
        for storage in self.storages.iter_mut() {
            storage.get_mut().clear();
        }

        for group in self.groups.iter_mut() {
            group.len = 0;
        }
    }

    pub(crate) fn borrow_with_info(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRef<ComponentStorage>, Option<GroupInfo>)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked(info.storage_index).borrow(),
                self.group_info.get(info.group_info_index).map(|info| {
                    let group_index = self.families.get_unchecked(info.family_index).start;
                    let group = NonNull::from(self.groups.get_unchecked(group_index));
                    GroupInfo::new(group, info.group_offset, info.storage_mask)
                }),
            )
        })
    }

    pub(crate) fn borrow_with_info_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, Option<GroupInfo>)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked(info.storage_index).borrow_mut(),
                self.group_info.get(info.group_info_index).map(|info| {
                    let group_index = self.families.get_unchecked(info.family_index).start;
                    let group = NonNull::from(self.groups.get_unchecked(group_index));
                    GroupInfo::new(group, info.group_offset, info.storage_mask)
                }),
            )
        })
    }

    pub(crate) fn borrow_with_family_mask_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, FamilyMask)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (self.storages.get_unchecked(info.storage_index).borrow_mut(), info.family_mask)
        })
    }

    pub(crate) fn get_with_family_mask_mut(
        &mut self,
        type_id: &TypeId,
    ) -> Option<(&mut ComponentStorage, FamilyMask)> {
        let info = self.component_info.get(type_id)?;

        unsafe {
            Some((self.storages.get_unchecked_mut(info.storage_index).get_mut(), info.family_mask))
        }
    }

    pub(crate) fn get_with_masks_mut(
        &mut self,
        type_id: &TypeId,
    ) -> Option<(&mut ComponentStorage, FamilyMask, GroupMask)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked_mut(info.storage_index).get_mut(),
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
struct ComponentInfo {
    storage_index: usize,
    group_info_index: usize,
    family_mask: FamilyMask,
    group_mask: GroupMask,
}

#[derive(Clone, Copy)]
struct StorageGroupInfo {
    family_index: usize,
    group_offset: usize,
    storage_mask: StorageMask,
}

/// Example:
///
/// Say we have a family made of two groups:
/// - Group 0: (A, B)
/// - Group 1: (A, B, C, D)
///
/// For the group 1, we get such a `Group` struct:
///
/// ```text
/// storages: A B C D
///           ^   ^   ^
///           |   |   +-- end = begin + 4
///           |   +------ new_begin = begin + 2
///           +---------- begin
/// ```
#[derive(Clone, Copy)]
pub(crate) struct Group {
    begin: usize,
    new_begin: usize,
    end: usize,
    /// Number of entities grouped by this group. Components of grouped entities
    /// are aligned to the left in the storage.
    len: usize,
}

impl Group {
    pub fn storage_range(&self) -> Range<usize> {
        self.begin..self.end
    }

    pub fn new_storage_range(&self) -> Range<usize> {
        self.new_begin..self.end
    }

    pub fn include_mask(&self) -> QueryMask {
        QueryMask::new_include_group(self.end - self.begin)
    }

    pub fn exclude_mask(&self) -> QueryMask {
        QueryMask::new_exclude_group(self.new_begin - self.begin, self.end - self.begin)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
