use crate::{entity::Entity, group::Group, storage::SparseArray};

pub enum GroupStatus<'a> {
    Entities,
    Grouped(GroupInfo<'a>),
    Ungrouped,
}

#[derive(Copy, Clone, Debug)]
pub struct GroupInfo<'a> {
    group: &'a Group,
    subgroup_len: usize,
}

impl<'a> GroupInfo<'a> {
    pub fn new(group: &'a Group, subgroup_len: usize) -> Self {
        Self {
            group,
            subgroup_len,
        }
    }
}

pub trait ComponentView<'a> {
    type Data: 'a + Copy;
    type Output: 'a;

    unsafe fn group_status(&self) -> GroupStatus<'a>;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data);

    unsafe fn get_from_data(data: Self::Data, index: usize) -> Self::Output;
}
