use crate::registry::WorldId;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Group {
    world_id: WorldId,
    info: GroupInfo,
}

impl Group {
    pub fn new(world_id: WorldId, info: GroupInfo) -> Self {
        Self { world_id, info }
    }

    pub fn world_id(&self) -> WorldId {
        self.world_id
    }

    pub fn info(&self) -> &GroupInfo {
        &self.info
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GroupInfo {
    group_index: usize,
    subgroup_index: usize,
    subgroup_len: usize,
}

impl GroupInfo {
    pub fn new(group_index: usize, subgroup_index: usize, subgroup_len: usize) -> Self {
        Self {
            group_index,
            subgroup_index,
            subgroup_len,
        }
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
