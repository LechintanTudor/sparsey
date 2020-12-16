use crate::group::{Group, GroupSet};
use atomic_refcell::AtomicRefCell;
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Debug)]
pub struct Groups {
    indexes: HashMap<ComponentTypeId, SubgroupIndex>,
    groups: Box<[AtomicRefCell<GroupData>]>,
}

impl Groups {
    pub fn new(group_set: GroupSet) -> Groups {
        let mut indexes = HashMap::<ComponentTypeId, SubgroupIndex>::new();
        let mut groups = Vec::<AtomicRefCell<GroupData>>::new();

        for (i, group) in group_set.into_groups().into_vec().into_iter().enumerate() {
            let group = GroupData::new(group);

            for (j, components) in group
                .subgroup_ends
                .iter()
                .map(|&i| &group.components[..i])
                .enumerate()
            {
                for &component in components {
                    indexes.insert(component, SubgroupIndex::new(i, j));
                }
            }

            groups.push(AtomicRefCell::new(group));
        }

        Groups {
            indexes,
            groups: groups.into(),
        }
    }

    pub fn get_subgroup_index(&self, component: ComponentTypeId) -> Option<&SubgroupIndex> {
        self.indexes.get(&component)
    }

    pub unsafe fn borrow_unchecked(&self, index: usize) -> &GroupData {
        &*self.groups.get_unchecked(index).as_ptr()
    }

    pub unsafe fn borrow_mut_unchecked(&self, index: usize) -> &mut GroupData {
        &mut *self.groups.get_unchecked(index).as_ptr()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SubgroupIndex {
    group_index: usize,
    subgroup_index: usize,
}

impl SubgroupIndex {
    pub fn new(group_index: usize, subgroup_index: usize) -> Self {
        Self {
            group_index,
            subgroup_index,
        }
    }

    pub fn group_index(&self) -> usize {
        self.group_index
    }

    pub fn subgroup_index(&self) -> usize {
        self.subgroup_index
    }
}

#[derive(Debug)]
pub struct GroupData {
    components: Box<[TypeId]>,
    subgroup_ends: Box<[usize]>,
    subgroup_lengths: Box<[usize]>,
}

impl GroupData {
    pub fn new(group: Group) -> Self {
        let (components, subgroup_ends) = group.into_components_and_subgroup_ends();
        let subgroup_lengths = vec![0; subgroup_ends.len()].into_boxed_slice();

        Self {
            components,
            subgroup_ends,
            subgroup_lengths,
        }
    }

    pub fn split(&mut self) -> (&[TypeId], &[usize], &mut [usize]) {
        (
            &self.components,
            &self.subgroup_ends,
            &mut self.subgroup_lengths,
        )
    }

    pub fn components(&self) -> &[TypeId] {
        &self.components
    }

    pub fn subgroup_ends(&self) -> &[usize] {
        &self.subgroup_ends
    }

    pub fn subgroup_lengths(&self) -> &[usize] {
        &self.subgroup_lengths
    }

    pub fn subgroup_count(&self) -> usize {
        self.subgroup_ends.len()
    }
}
