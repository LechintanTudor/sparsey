#[derive(Debug)]
pub struct Group {
    components: Box<[TypeId]>,
    subgroup_arities: Box<[usize]>,
    subgroup_lengths: Box<[usize]>,
}

use super::GroupLayout;
use std::{any::TypeId, slice::SliceIndex};

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
