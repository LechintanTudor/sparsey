use crate::{
    data::{ParentGroup, ParentGroupInfo},
    group::{Group, WorldLayout},
};
use std::{any::TypeId, cell::UnsafeCell, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Debug)]
pub struct Groups {
    indexes: HashMap<ComponentTypeId, SubgroupIndex>,
    groups: Box<[UnsafeCell<Group>]>,
}

impl Groups {
    pub fn new(world_layout: WorldLayout) -> Groups {
        let mut indexes = HashMap::<ComponentTypeId, SubgroupIndex>::new();
        let mut groups = Vec::<UnsafeCell<Group>>::new();

        for (i, layout) in world_layout
            .into_group_layouts()
            .into_vec()
            .into_iter()
            .enumerate()
        {
            let group = Group::new(layout);

            for (j, components) in group
                .subgroup_arities()
                .iter()
                .map(|&i| &group.components()[..i])
                .enumerate()
            {
                for &component in components {
                    indexes.insert(component, SubgroupIndex::new(i, j));
                }
            }

            groups.push(UnsafeCell::new(group));
        }

        Groups {
            indexes,
            groups: groups.into(),
        }
    }

    pub fn get_subgroup_index(&self, component: ComponentTypeId) -> Option<&SubgroupIndex> {
        self.indexes.get(&component)
    }

    pub fn get_parent_group(&self, component: ComponentTypeId) -> ParentGroup {
        match self.get_subgroup_index(component) {
            Some(subgroup) => unsafe {
                let group = self.get_unchecked(subgroup.group_index);
                let subgroup_len = *group
                    .subgroup_lengths()
                    .get_unchecked(subgroup.subgroup_index);

                ParentGroup::Some(ParentGroupInfo::new(group, subgroup_len))
            },
            None => ParentGroup::None,
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &Group {
        &*self.groups.get_unchecked(index).get()
    }

    pub unsafe fn get_mut_unchecked(&self, index: usize) -> &mut Group {
        &mut *self.groups.get_unchecked(index).get()
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
