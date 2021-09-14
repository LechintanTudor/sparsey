use crate::components::Group;
use crate::group::GroupMask;
use std::ops::Range;
use std::ptr;

/// Tracks the group to which a component storage belongs.
#[derive(Clone, Copy)]
pub struct GroupInfo<'a> {
    group_family: &'a [Group],
    group_offset: usize,
    storage_mask: u16,
}

impl<'a> GroupInfo<'a> {
    pub(crate) const fn new(
        group_family: &'a [Group],
        group_offset: usize,
        storage_mask: u16,
    ) -> Self {
        Self {
            group_family,
            group_offset,
            storage_mask,
        }
    }
}

/// Tracks the group to which a multiple component storages belong.
#[derive(Copy, Clone, Default)]
pub struct CombinedGroupInfo<'a> {
    group_family: Option<&'a [Group]>,
    max_group_offset: usize,
    group_mask: u16,
}

impl<'a> CombinedGroupInfo<'a> {
    pub fn combine(self, group_info: GroupInfo<'a>) -> Option<Self> {
        match self.group_family {
            Some(group_family) => {
                ptr::eq(group_family, group_info.group_family).then(|| CombinedGroupInfo {
                    group_family: Some(group_family),
                    max_group_offset: self.max_group_offset.max(group_info.group_offset),
                    group_mask: self.group_mask | group_info.storage_mask,
                })
            }
            None => Some(CombinedGroupInfo {
                group_family: Some(group_info.group_family),
                max_group_offset: group_info.group_offset,
                group_mask: group_info.storage_mask,
            }),
        }
    }
}

fn common_group_family<'a>(group_families: &[Option<&'a [Group]>]) -> Option<&'a [Group]> {
    let mut group_family: Option<&[Group]> = None;

    for &gf in group_families {
        if let Some(gf) = gf {
            match group_family {
                Some(group_family) => {
                    if !ptr::eq(group_family, gf) {
                        return None;
                    }
                }
                None => group_family = Some(gf),
            }
        }
    }

    group_family
}

/// Returns the range of elements the storages have in common if the
/// `CombinedGroupInfo`s form a group.
pub fn group_range(
    base: CombinedGroupInfo,
    include: CombinedGroupInfo,
    exclude: CombinedGroupInfo,
) -> Option<Range<usize>> {
    let group_family = common_group_family(&[
        base.group_family,
        include.group_family,
        exclude.group_family,
    ])?;

    let max_group_offset = base
        .max_group_offset
        .max(include.max_group_offset)
        .max(exclude.max_group_offset);

    let group_mask = GroupMask::new(base.group_mask | include.group_mask, exclude.group_mask);
    let group = &group_family[max_group_offset as usize];

    println!(
        "G1: {:#018b} {:#018b}",
        group_mask.include, group_mask.exclude
    );
    println!(
        "G2: {:#018b} {:#018b}",
        group.exclude_mask().include,
        group.exclude_mask().exclude
    );

    if group_mask == group.include_mask() {
        Some(0..group.len())
    } else if group_mask == group.exclude_mask() {
        let prev_group = &group_family[(max_group_offset - 1) as usize];
        Some(group.len()..prev_group.len())
    } else {
        None
    }
}
