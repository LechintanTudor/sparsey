pub(crate) const MAX_GROUP_FAMILIES: usize = 16;

pub(crate) fn iter_group_family_indexes(family_mask: u16) -> impl Iterator<Item = usize> {
    (0..MAX_GROUP_FAMILIES).filter(move |i| (family_mask & (1 << i)) != 0)
}
