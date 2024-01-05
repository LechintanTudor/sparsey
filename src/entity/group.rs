use crate::entity::QueryMask;

#[derive(Clone, Copy, Debug)]
pub struct Group {
    pub metadata: GroupMetadata,
    pub len: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct GroupMetadata {
    pub start: usize,
    pub new_start: usize,
    pub end: usize,
    pub include_mask: QueryMask,
    pub exclude_mask: QueryMask,
}
