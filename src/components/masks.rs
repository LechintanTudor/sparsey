pub(crate) type FamilyMask = u32;
pub(crate) type GroupMask = u32;

pub(crate) fn new_group_mask(index: usize, arity: usize, family_arity: usize) -> GroupMask {
    ((1 << (family_arity + 1 - arity)) - 1) << index
}

pub(crate) fn iter_bit_indexes(mask: u32) -> BitIndexIter {
    BitIndexIter::new(mask)
}

#[derive(Clone, Debug)]
pub(crate) struct BitIndexIter {
    mask: u32,
    offset: u32,
}

impl BitIndexIter {
    fn new(mask: u32) -> BitIndexIter {
        BitIndexIter { mask, offset: 0 }
    }
}

impl Iterator for BitIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let trailing_zeros = self.mask.trailing_zeros();

        if trailing_zeros == u32::BITS {
            return None;
        }

        self.mask >>= trailing_zeros + 1;
        self.offset += trailing_zeros;

        let index = self.offset as usize;
        self.offset += 1;

        Some(index)
    }
}
