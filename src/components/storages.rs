use crate::components::Component;
use crate::group::{GroupInfo, GroupMask};
use crate::layout::Layout;
use crate::storage::{ComponentStorage, Entity};
use crate::utils::UnsafeUnwrap;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::{Entry, IterMut as HashMapIterMut};
use std::mem;
use std::ops::Range;

pub type FamilyMask = u16;

#[derive(Default)]
pub struct ComponentStorages {
    storages: Vec<AtomicRefCell<ComponentStorage>>,
    component_info: FxHashMap<TypeId, ComponentInfo>,
    group_info: Vec<ComponentGroupInfo>,
    groups: Vec<Group>,
    families: Vec<Range<usize>>,
}

impl ComponentStorages {
    pub unsafe fn new(
        layout: &Layout,
        spare_storages: &mut FxHashMap<TypeId, ComponentStorage>,
    ) -> Self {
        let mut component_info = FxHashMap::default();
        let mut storages = Vec::new();
        let mut group_info = Vec::new();
        let mut groups = Vec::new();
        let mut families = Vec::new();

        // Iterate group families.
        for (family_index, layout) in layout.group_families().iter().enumerate() {
            let first_group_index = groups.len();
            let first_storage_index = storages.len();

            let components = layout.components();
            let mut prev_arity = 0_usize;

            // Iterate groups in a group family.
            for (group_offset, &arity) in layout.group_arities().iter().enumerate() {
                let new_storage_index = storages.len();

                // Iterate new components in a group.
                for (component_offset, component) in
                    (&components[prev_arity..arity]).iter().enumerate()
                {
                    let type_id = component.type_id();

                    component_info.insert(
                        type_id,
                        ComponentInfo {
                            storage_index: storages.len(),
                            group_info_index: group_info.len(),
                            family_mask: 1 << family_index,
                        },
                    );

                    group_info.push(ComponentGroupInfo {
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
                },
            );

            storages.push(AtomicRefCell::new(storage));
        }

        Self {
            component_info,
            storages,
            group_info,
            families,
            groups,
        }
    }

    pub fn into_storages(mut self) -> FxHashMap<TypeId, ComponentStorage> {
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

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        unsafe {
            self.register_with(TypeId::of::<T>(), ComponentStorage::new::<T>);
        }
    }

    pub unsafe fn register_with<F>(&mut self, type_id: TypeId, storage_builder: F)
    where
        F: FnOnce() -> ComponentStorage,
    {
        if let Entry::Vacant(entry) = self.component_info.entry(type_id) {
            entry.insert(ComponentInfo {
                storage_index: self.storages.len(),
                group_info_index: usize::MAX,
                family_mask: 0,
            });

            self.storages.push(AtomicRefCell::new(storage_builder()));
        }
    }

    pub fn is_registered(&self, type_id: &TypeId) -> bool {
        self.component_info.contains_key(type_id)
    }

    /// Returns an iterator over all storages and the `TypeId`s of the
    /// components they hold.
    pub fn iter(&mut self) -> ComponentStoragesIter {
        ComponentStoragesIter::new(self)
    }

    pub unsafe fn group_components<'a, E>(&mut self, family_index: usize, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
    {
        let groups = self
            .groups
            .get_unchecked_mut(self.families.get_unchecked(family_index).clone());
        let storages = &mut self.storages;

        entities.into_iter().for_each(|&entity| {
            for group in groups.iter_mut() {
                let status = get_group_status(
                    storages.get_unchecked_mut(group.new_storage_range()),
                    group.len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => (),
                    GroupStatus::Ungrouped => {
                        group_components(
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

    pub unsafe fn ungroup_components<'a, E>(&mut self, family_index: usize, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
    {
        let groups = self
            .groups
            .get_unchecked_mut(self.families.get_unchecked(family_index).clone());
        let storages = &mut self.storages;

        entities.into_iter().for_each(|&entity| {
            let mut ungroup_start = 0_usize;
            let mut ungroup_len = 0_usize;

            for (i, group) in groups.iter_mut().enumerate() {
                let status = get_group_status(
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

            for group in groups.get_unchecked_mut(ungroup_range).iter_mut().rev() {
                ungroup_components(
                    storages.get_unchecked_mut(group.storage_range()),
                    &mut group.len,
                    entity,
                );
            }
        });
    }

    pub fn group_all_components<'a, E>(&mut self, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        if !self.families.is_empty() {
            let entities = entities.into_iter();

            for i in 0..(self.families.len() - 1) {
                unsafe {
                    self.group_components(i, entities.clone());
                }
            }

            unsafe {
                self.group_components(self.families.len() - 1, entities);
            }
        }
    }

    pub fn ungroup_all_components<'a, E>(&mut self, entities: E)
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        if !self.families.is_empty() {
            let entities = entities.into_iter();

            for i in 0..(self.families.len() - 1) {
                unsafe {
                    self.ungroup_components(i, entities.clone());
                }
            }

            unsafe {
                self.ungroup_components(self.families.len() - 1, entities);
            }
        }
    }

    /// Removes all entities and components from the storages.
    pub fn clear(&mut self) {
        for storage in self.storages.iter_mut() {
            storage.get_mut().clear();
        }

        for group in self.groups.iter_mut() {
            group.len = 0;
        }
    }

    pub fn borrow_with_info(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRef<ComponentStorage>, Option<GroupInfo>)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked(info.storage_index).borrow(),
                self.group_info.get(info.group_info_index).map(|info| {
                    GroupInfo::new(
                        self.groups
                            .get_unchecked(self.families.get_unchecked(info.family_index).clone()),
                        info.group_offset,
                        info.storage_mask,
                    )
                }),
            )
        })
    }

    pub fn borrow_with_info_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, Option<GroupInfo>)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked(info.storage_index).borrow_mut(),
                self.group_info.get(info.group_info_index).map(|info| {
                    GroupInfo::new(
                        self.groups
                            .get_unchecked(self.families.get_unchecked(info.family_index).clone()),
                        info.group_offset,
                        info.storage_mask,
                    )
                }),
            )
        })
    }

    pub fn borrow_with_family_mask_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<(AtomicRefMut<ComponentStorage>, FamilyMask)> {
        self.component_info.get(type_id).map(|info| unsafe {
            (
                self.storages.get_unchecked(info.storage_index).borrow_mut(),
                info.family_mask,
            )
        })
    }

    pub fn get_mut(&mut self, type_id: &TypeId) -> Option<&mut ComponentStorage> {
        let info = self.component_info.get(type_id)?;

        unsafe {
            Some(
                self.storages
                    .get_unchecked_mut(info.storage_index)
                    .get_mut(),
            )
        }
    }

    pub fn get_with_family_mask_mut(
        &mut self,
        type_id: &TypeId,
    ) -> Option<(&mut ComponentStorage, FamilyMask)> {
        let info = self.component_info.get(type_id)?;

        unsafe {
            Some((
                self.storages
                    .get_unchecked_mut(info.storage_index)
                    .get_mut(),
                info.family_mask,
            ))
        }
    }

    pub fn get_family_mask(&self, type_id: &TypeId) -> FamilyMask {
        self.component_info
            .get(type_id)
            .map(|info| info.family_mask)
            .unwrap_or(0)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
        self.storages.iter_mut().map(AtomicRefCell::get_mut)
    }
}

/// Iterator over all storages in `ComponentStorages` and the `TypeId`s of the
/// components they hold.
pub struct ComponentStoragesIter<'a> {
    inner: HashMapIterMut<'a, TypeId, ComponentInfo>,
    storages: *const AtomicRefCell<ComponentStorage>,
}

impl<'a> ComponentStoragesIter<'a> {
    fn new(component_storages: &'a mut ComponentStorages) -> Self {
        Self {
            inner: component_storages.component_info.iter_mut(),
            storages: component_storages.storages.as_ptr(),
        }
    }
}

impl<'a> Iterator for ComponentStoragesIter<'a> {
    type Item = (&'a TypeId, &'a ComponentStorage);

    fn next(&mut self) -> Option<Self::Item> {
        let (type_id, info) = self.inner.next()?;
        let storage = unsafe { &*(*self.storages.add(info.storage_index)).as_ptr() };
        Some((type_id, storage))
    }
}

#[derive(Clone, Copy)]
struct ComponentInfo {
    storage_index: usize,
    group_info_index: usize,
    family_mask: u16,
}

#[derive(Clone, Copy)]
struct ComponentGroupInfo {
    family_index: usize,
    group_offset: usize,
    storage_mask: u16,
}

#[derive(Clone, Copy)]
pub(crate) struct Group {
    begin: usize,
    new_begin: usize,
    end: usize,
    len: usize,
}

impl Group {
    pub fn storage_range(&self) -> Range<usize> {
        self.begin..self.end
    }

    pub fn new_storage_range(&self) -> Range<usize> {
        self.new_begin..self.end
    }

    pub fn include_mask(&self) -> GroupMask {
        GroupMask::include(self.end - self.begin)
    }

    pub fn exclude_mask(&self) -> GroupMask {
        GroupMask::exclude(self.new_begin - self.begin, self.end - self.begin)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

fn get_group_status(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: usize,
    entity: Entity,
) -> GroupStatus {
    match storages.split_first_mut() {
        Some((first, others)) => {
            let status = match first.get_mut().get_index(entity) {
                Some(index) => {
                    if index < group_len {
                        GroupStatus::Grouped
                    } else {
                        GroupStatus::Ungrouped
                    }
                }
                None => return GroupStatus::MissingComponents,
            };

            if others
                .iter_mut()
                .all(|storage| storage.get_mut().contains(entity))
            {
                status
            } else {
                GroupStatus::MissingComponents
            }
        }
        None => GroupStatus::Grouped,
    }
}

unsafe fn group_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
        let index = storage.get_index(entity).unsafe_unwrap();
        storage.swap(index, *group_len);
    }

    *group_len += 1;
}

unsafe fn ungroup_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
            let index = storage.get_index(entity).unsafe_unwrap();
            storage.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
