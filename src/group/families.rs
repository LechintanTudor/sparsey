pub(crate) const MAX_GROUP_FAMILIES: usize = 16;

/// Set of group families indexes used by a component set.
#[derive(Clone, Copy, Default, Debug)]
pub struct GroupFamilyIndexes {
    used: [bool; MAX_GROUP_FAMILIES],
}

impl GroupFamilyIndexes {
    pub(crate) unsafe fn insert_unchecked(&mut self, family_index: usize) {
        *self.used.get_unchecked_mut(family_index) = true;
    }

    /// Returns an iterator over the used group family indexes.
    pub fn indexes(&self) -> GroupFamilyIndexIter {
        GroupFamilyIndexIter::new(&self.used)
    }
}

/// Iterator over the group family indexes of a component set.
#[derive(Clone, Copy)]
pub struct GroupFamilyIndexIter<'a> {
    index: usize,
    used: &'a [bool; MAX_GROUP_FAMILIES],
}

impl<'a> GroupFamilyIndexIter<'a> {
    const fn new(used: &'a [bool; MAX_GROUP_FAMILIES]) -> Self {
        Self { index: 0, used }
    }
}

impl<'a> Iterator for GroupFamilyIndexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < MAX_GROUP_FAMILIES {
            let current_index = self.index;
            self.index += 1;

            if self.used[current_index] {
                return Some(current_index);
            }
        }

        None
    }
}
