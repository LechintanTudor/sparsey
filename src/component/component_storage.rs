use crate::component::{
    group, ungroup_all, Component, ComponentSparseSet, Group, GroupInfo, GroupLayout, GroupMask,
    GroupMetadata, NonZeroStorageMask, QueryGroupInfo, QueryMask, StorageMask, View, ViewMut,
};
use crate::entity::Entity;
use atomic_refcell::AtomicRefCell;
use rustc_hash::FxHashMap;
use std::any::{self, TypeId};
use std::collections::hash_map::Entry;
use std::ops::Range;
use std::{cmp, mem};

#[derive(Default, Debug)]
pub(crate) struct ComponentStorage {
    pub(crate) groups: Vec<Group>,
    pub(crate) metadata: FxHashMap<TypeId, ComponentMetadata>,
    pub(crate) components: Vec<AtomicRefCell<ComponentSparseSet>>,
}

impl ComponentStorage {
    #[must_use]
    pub unsafe fn new(
        entities: &[Entity],
        layout: &GroupLayout,
        mut sparse_sets: FxHashMap<TypeId, ComponentSparseSet>,
    ) -> Self {
        let mut groups = Vec::new();
        let mut metadata = FxHashMap::default();
        let mut components = Vec::new();

        for family in layout.families() {
            let storage_start = components.len();
            let group_start = groups.len();
            let group_end = group_start + family.arities().len();

            let mut prev_arity = 0;

            for &arity in family.arities() {
                let storage_end = storage_start + arity;
                let new_group_start = groups.len();

                groups.push(Group {
                    metadata: GroupMetadata {
                        storage_start,
                        new_storage_start: storage_start + prev_arity,
                        storage_end,
                        skip_mask: GroupMask::skip_from_to(new_group_start, group_end),
                        include_mask: QueryMask::include(arity),
                        exclude_mask: QueryMask::exclude(prev_arity, arity),
                    },
                    len: 0,
                });

                for local_storage_index in prev_arity..arity {
                    let component = &family.components()[local_storage_index];

                    metadata.insert(
                        component.type_id(),
                        ComponentMetadata {
                            storage_index: components.len(),
                            insert_mask: GroupMask::from_to(group_start, group_end),
                            delete_mask: GroupMask::from_to(new_group_start, group_end),
                            group_info: Some(GroupInfo {
                                group_start: group_start as u8,
                                group_end: groups.len() as u8,
                                storage_mask: NonZeroStorageMask::single(local_storage_index),
                            }),
                        },
                    );

                    let sparse_set = sparse_sets
                        .remove(&component.type_id())
                        .unwrap_or_else(|| component.create_sparse_set());

                    components.push(AtomicRefCell::new(sparse_set));
                }

                prev_arity = arity;
            }
        }

        for (type_id, sparse_set) in sparse_sets {
            metadata.insert(
                type_id,
                ComponentMetadata {
                    storage_index: components.len(),
                    insert_mask: GroupMask::default(),
                    delete_mask: GroupMask::default(),
                    group_info: None,
                },
            );

            components.push(AtomicRefCell::new(sparse_set));
        }

        let group_mask = GroupMask::from_to(0, groups.len());

        for &entity in entities {
            unsafe {
                group(&mut components, &mut groups, group_mask, entity);
            }
        }

        Self {
            groups,
            metadata,
            components,
        }
    }

    pub fn into_sparse_sets(mut self) -> FxHashMap<TypeId, ComponentSparseSet> {
        let mut sparse_sets = FxHashMap::default();

        for (type_id, metadata) in self.metadata {
            let sparse_set = mem::replace(
                self.components[metadata.storage_index].get_mut(),
                ComponentSparseSet::new::<()>(),
            );

            sparse_sets.insert(type_id, sparse_set);
        }

        sparse_sets
    }

    pub fn register<T>(&mut self) -> bool
    where
        T: Component,
    {
        let Entry::Vacant(entry) = self.metadata.entry(TypeId::of::<T>()) else {
            return false;
        };

        entry.insert(ComponentMetadata {
            storage_index: self.components.len(),
            insert_mask: GroupMask::default(),
            delete_mask: GroupMask::default(),
            group_info: None,
        });

        self.components
            .push(AtomicRefCell::new(ComponentSparseSet::new::<T>()));

        true
    }

    #[must_use]
    pub fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.metadata.contains_key(&TypeId::of::<T>())
    }

    pub fn strip(&mut self, entity: Entity) {
        unsafe {
            ungroup_all(&mut self.components, &mut self.groups, entity);
        }

        for sparse_set in &mut self.components {
            sparse_set.get_mut().delete_dyn(entity);
        }
    }

    pub fn clear(&mut self) {
        for group in &mut self.groups {
            group.len = 0;
        }

        for sparse_set in &mut self.components {
            sparse_set.get_mut().clear();
        }
    }

    #[must_use]
    pub fn borrow<T>(&self) -> View<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe {
            View::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow(),
            )
        }
    }

    #[must_use]
    pub fn borrow_mut<T>(&self) -> ViewMut<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe {
            ViewMut::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow_mut(),
            )
        }
    }

    #[must_use]
    pub fn borrow_with_group_info<T>(&self) -> (View<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        let view = unsafe {
            View::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow(),
            )
        };

        (view, metadata.group_info)
    }

    #[must_use]
    pub fn borrow_with_group_info_mut<T>(&self) -> (ViewMut<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        let view = unsafe {
            ViewMut::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow_mut(),
            )
        };

        (view, metadata.group_info)
    }

    #[must_use]
    pub unsafe fn group_range(
        &self,
        include: &QueryGroupInfo,
        exclude: &QueryGroupInfo,
    ) -> Option<Range<usize>> {
        type Info = QueryGroupInfo;

        match (include, exclude) {
            (Info::One(view), Info::Empty) => Some(0..view.len),
            (Info::Many(include), Info::Empty) => self.include_group_range(*include),
            (include, exclude) => {
                let include = include.group_info()?;
                let exclude = exclude.group_info()?;
                self.exclude_group_range(include, exclude)
            }
        }
    }

    #[must_use]
    unsafe fn include_group_range(&self, include: GroupInfo) -> Option<Range<usize>> {
        let group = unsafe {
            self.groups
                .get_unchecked(usize::from(include.group_end) - 1)
        };

        let mask = QueryMask {
            include: include.storage_mask.into(),
            exclude: StorageMask::EMPTY,
        };

        (mask == group.metadata.include_mask).then_some(0..group.len)
    }

    #[must_use]
    unsafe fn exclude_group_range(
        &self,
        include: GroupInfo,
        exclude: GroupInfo,
    ) -> Option<Range<usize>> {
        if include.group_start != exclude.group_start {
            return None;
        }

        let group_end = cmp::max(include.group_end, exclude.group_end);
        let child_group = unsafe { self.groups.get_unchecked(usize::from(group_end) - 1) };

        let mask = QueryMask {
            include: include.storage_mask.into(),
            exclude: exclude.storage_mask.into(),
        };

        if mask != child_group.metadata.exclude_mask {
            return None;
        }

        let parent_group = unsafe { self.groups.get_unchecked(usize::from(group_end) - 2) };
        Some(child_group.len..parent_group.len)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ComponentMetadata {
    pub storage_index: usize,
    pub insert_mask: GroupMask,
    pub delete_mask: GroupMask,
    pub group_info: Option<GroupInfo>,
}

#[cold]
#[inline(never)]
pub(crate) fn panic_missing_comp<T>() -> ! {
    panic!("Component '{}' was not registered", any::type_name::<T>());
}
