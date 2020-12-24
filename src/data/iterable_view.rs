use crate::{entity::Entity, group::Group, storage::SparseArray};

#[derive(Copy, Clone, Debug)]
pub enum ParentGroup<'a> {
    Some(ParentGroupInfo<'a>),
    None,
    Entities,
}

#[derive(Copy, Clone, Debug)]
pub struct ParentGroupInfo<'a> {
    group: &'a Group,
    subgroup_len: usize,
}

impl<'a> ParentGroupInfo<'a> {
    pub fn new(group: &'a Group, subgroup_len: usize) -> Self {
        Self {
            group,
            subgroup_len,
        }
    }

    pub fn group(&self) -> &Group {
        self.group
    }

    pub fn subgroup_len(&self) -> usize {
        self.subgroup_len
    }
}

pub trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    unsafe fn parent_group(&self) -> ParentGroup<'a>;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output>;
}
