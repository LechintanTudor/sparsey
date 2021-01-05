use crate::entity::Entity;
use crate::storage::SparseArray;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ParentGroup {
    group_id: usize,
    subgroup_index: usize,
    subgroup_len: usize,
}

impl ParentGroup {
    pub fn new(group_id: usize, subgroup_index: usize, subgroup_len: usize) -> Self {
        Self {
            group_id,
            subgroup_index,
            subgroup_len,
        }
    }

    pub fn group_id(&self) -> usize {
        self.group_id
    }

    pub fn subgroup_index(&self) -> usize {
        self.subgroup_index
    }

    pub fn subgroup_len(&self) -> usize {
        self.subgroup_len
    }
}

pub trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    fn parent_group(&self) -> Option<ParentGroup>;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output>;
}
