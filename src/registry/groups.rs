use crate::group::{Group, WorldLayout};
use atomic_refcell::AtomicRefCell;
use std::{any::TypeId, collections::HashMap};

type ComponentTypeId = TypeId;

#[derive(Debug)]
pub struct Groups {
    indexes: HashMap<ComponentTypeId, SubgroupIndex>,
    groups: Box<[AtomicRefCell<Group>]>,
}

impl Groups {
    pub fn new(world_layout: WorldLayout) -> Groups {
        let mut indexes = HashMap::<ComponentTypeId, SubgroupIndex>::new();
        let mut groups = Vec::<AtomicRefCell<Group>>::new();

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

    pub unsafe fn get_unchecked(&self, index: usize) -> &Group {
        &*self.groups.get_unchecked(index).as_ptr()
    }

    pub unsafe fn get_mut_unchecked(&self, index: usize) -> &mut Group {
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
