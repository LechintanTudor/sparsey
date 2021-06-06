pub(crate) const MAX_GROUP_FAMILIES: usize = 16;

#[derive(Clone, Copy, Default, Debug)]
pub struct UsedGroupFamilies {
	used: [bool; MAX_GROUP_FAMILIES],
}

impl UsedGroupFamilies {
	pub(crate) unsafe fn insert_unchecked(&mut self, family_index: usize) {
		*self.used.get_unchecked_mut(family_index) = true;
	}

	pub(crate) fn indexes(&self) -> impl Iterator<Item = usize> + '_ {
		(0..MAX_GROUP_FAMILIES)
			.into_iter()
			.filter(move |&i| self.used[i])
	}
}
