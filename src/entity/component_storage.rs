use crate::entity::{
    group, ungroup_all, Comp, CompMut, Component, ComponentSparseSet, Entity, Group, GroupInfo,
    GroupLayout, GroupMask, GroupMetadata, QueryMask, StorageMask,
};
use atomic_refcell::AtomicRefCell;
use rustc_hash::FxHashMap;
use std::any::{self, TypeId};
use std::collections::hash_map::Entry;
use std::mem;

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
                            group_end: groups.len(),
                            storage_mask: StorageMask::single(local_storage_index),
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
                    group_end: 0,
                    storage_mask: StorageMask::default(),
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
            group_end: 0,
            storage_mask: StorageMask::default(),
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

    pub fn delete_all(&mut self, entity: Entity) {
        unsafe {
            ungroup_all(&mut self.components, &mut self.groups, entity);
        }

        for sparse_set in &mut self.components {
            sparse_set.get_mut().delete_dyn(entity);
        }
    }

    #[must_use]
    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        let group_info = (metadata.storage_mask.0 != 0).then(|| unsafe {
            GroupInfo::new(
                self.groups.get_unchecked(0..metadata.group_end),
                metadata.storage_mask,
            )
        });

        unsafe {
            Comp::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow(),
                group_info,
            )
        }
    }

    #[must_use]
    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        let group_info = (metadata.storage_mask.0 != 0).then(|| unsafe {
            GroupInfo::new(
                self.groups.get_unchecked(0..metadata.group_end),
                metadata.storage_mask,
            )
        });

        unsafe {
            CompMut::new(
                self.components
                    .get_unchecked(metadata.storage_index)
                    .borrow_mut(),
                group_info,
            )
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ComponentMetadata {
    pub storage_index: usize,
    pub insert_mask: GroupMask,
    pub delete_mask: GroupMask,
    pub group_end: usize,
    pub storage_mask: StorageMask,
}

#[cold]
#[inline(never)]
pub(crate) fn panic_missing_comp<T>() -> ! {
    panic!("Component '{}' was not registered", any::type_name::<T>());
}
