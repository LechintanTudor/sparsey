use crate::group::{GroupLayout, WorldLayout};
use atomic_refcell::AtomicRefCell;
use std::{any::TypeId, collections::HashMap, slice::SliceIndex};

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
                .subgroup_arities
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

#[derive(Debug)]
pub struct Group {
    components: Box<[TypeId]>,
    subgroup_arities: Box<[usize]>,
    subgroup_lengths: Box<[usize]>,
}

impl Group {
    pub fn new(layout: GroupLayout) -> Self {
        let (components, subgroup_arities) = layout.into_components_and_arities();
        let subgroup_lengths = vec![0; subgroup_arities.len()].into_boxed_slice();

        Self {
            components,
            subgroup_arities,
            subgroup_lengths,
        }
    }

    pub fn split(&mut self) -> (&[TypeId], &[usize], &mut [usize]) {
        (
            &self.components,
            &self.subgroup_arities,
            &mut self.subgroup_lengths,
        )
    }

    pub fn iter_subgroups_mut<I>(
        &mut self,
        range: I,
    ) -> impl DoubleEndedIterator<Item = (usize, &mut usize)>
    where
        I: SliceIndex<[usize], Output = [usize]> + Clone,
    {
        (&self.subgroup_arities[range.clone()])
            .iter()
            .zip((&mut self.subgroup_lengths[range]).iter_mut())
            .map(|(a, l)| (*a, l))
    }

    pub fn components(&self) -> &[TypeId] {
        &self.components
    }

    pub fn subgroup_arities(&self) -> &[usize] {
        &self.subgroup_arities
    }

    pub fn subgroup_lengths(&self) -> &[usize] {
        &self.subgroup_lengths
    }

    pub fn subgroup_count(&self) -> usize {
        self.subgroup_arities.len()
    }
}
