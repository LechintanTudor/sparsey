use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WorldId(NonZeroU64);

impl WorldId {
    pub fn new() -> WorldId {
        static CURRENT_ID: AtomicU64 = AtomicU64::new(1);
        WorldId(NonZeroU64::new(CURRENT_ID.fetch_add(1, Ordering::Relaxed)).unwrap())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupInfo {
    world_id: WorldId,
    group_index: usize,
    subgroup_index: usize,
    subgroup_len: usize,
}

impl GroupInfo {
    pub fn new(
        world_id: WorldId,
        group_index: usize,
        subgroup_index: usize,
        subgroup_len: usize,
    ) -> Self {
        Self {
            world_id,
            group_index,
            subgroup_index,
            subgroup_len,
        }
    }

    pub fn world_id(&self) -> WorldId {
        self.world_id
    }

    pub fn group_index(&self) -> usize {
        self.group_index
    }

    pub fn subgroup_index(&self) -> usize {
        self.subgroup_index
    }

    pub fn subgroup_len(&self) -> usize {
        self.subgroup_len
    }
}
