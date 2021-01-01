use super::GroupLayout;
use std::{
    any::TypeId,
    slice::SliceIndex,
    sync::atomic::{AtomicUsize, Ordering},
};

static CURRENT_GROUP_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Group {
    id: usize,
    components: Box<[TypeId]>,
    subgroup_arities: Box<[usize]>,
    subgroup_lengths: Box<[usize]>,
}

impl Group {
    pub fn new(layout: GroupLayout) -> Self {
        let (components, subgroup_arities) = layout.into_components_and_arities();
        let subgroup_lengths = vec![0; subgroup_arities.len()].into_boxed_slice();

        Self {
            id: CURRENT_GROUP_ID.fetch_add(1, Ordering::Relaxed),
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

    pub fn id(&self) -> usize {
        self.id
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
