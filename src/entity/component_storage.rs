use crate::entity::{
    Comp, CompMut, Component, ComponentSparseSet, Group, GroupLayout, GroupMask, GroupMetadata,
};
use atomic_refcell::AtomicRefCell;
use rustc_hash::FxHashMap;
use std::any;
use std::any::TypeId;
use std::collections::hash_map::Entry;

#[derive(Default, Debug)]
pub struct ComponentStorage {
    groups: Vec<Group>,
    metadata: FxHashMap<TypeId, ComponentMetadata>,
    components: Vec<AtomicRefCell<ComponentSparseSet>>,
}

impl ComponentStorage {
    pub fn new(layout: &GroupLayout) -> Self {
        let mut groups = Vec::new();
        let mut metadata = FxHashMap::default();
        let mut components = Vec::new();

        for family in layout.families() {
            let storage_start = components.len();
            let storage_end = storage_start + family.components().len();
            let group_start = groups.len();
            let group_end = group_start + family.arities().len();

            let mut prev_arity = 0;

            for &arity in family.arities() {
                let new_group_start = groups.len();

                groups.push(Group {
                    metadata: GroupMetadata {
                        storage_start,
                        new_storage_start: components.len(),
                        storage_end,
                        skip_mask: GroupMask::skip_from_to(new_group_start, group_end),
                    },
                    len: 0,
                });

                for component in &family.components()[prev_arity..arity] {
                    metadata.insert(
                        component.type_id(),
                        ComponentMetadata {
                            index: components.len(),
                            insert_mask: GroupMask::from_to(group_start, group_end),
                            delete_mask: GroupMask::from_to(new_group_start, group_end),
                        },
                    );

                    components.push(AtomicRefCell::new(component.create_sparse_set()));
                }

                prev_arity = arity;
            }
        }

        Self {
            groups,
            metadata,
            components,
        }
    }

    pub fn register<T>(&mut self) -> bool
    where
        T: Component,
    {
        let Entry::Vacant(entry) = self.metadata.entry(TypeId::of::<T>()) else {
            return false;
        };

        entry.insert(ComponentMetadata {
            index: self.components.len(),
            insert_mask: GroupMask::default(),
            delete_mask: GroupMask::default(),
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

    #[must_use]
    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe { Comp::<T>::new(self.components.get_unchecked(metadata.index).borrow()) }
    }

    #[must_use]
    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe { CompMut::<T>::new(self.components.get_unchecked(metadata.index).borrow_mut()) }
    }
}

#[derive(Clone, Copy, Debug)]
struct ComponentMetadata {
    index: usize,
    insert_mask: GroupMask,
    delete_mask: GroupMask,
}

#[cold]
#[inline(never)]
fn panic_missing_comp<T>() -> ! {
    panic!("Component '{}' was not registered", any::type_name::<T>());
}
