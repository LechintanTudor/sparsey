pub(crate) const MAX_GROUP_FAMILIES: usize = 16;

#[derive(Clone, Copy, Debug)]
pub struct UsedGroupFamilies {
	used: [bool; MAX_GROUP_FAMILIES],
}

impl UsedGroupFamilies {
	pub(crate) const fn new() -> Self {
		Self {
			used: [false; MAX_GROUP_FAMILIES],
		}
	}

	pub(crate) unsafe fn add_unchecked(&mut self, index: usize) {
		*self.used.get_unchecked_mut(index) = true;
	}

	pub(crate) fn indexes(&self) -> impl Iterator<Item = usize> + '_ {
		(0..MAX_GROUP_FAMILIES)
			.into_iter()
			.filter(move |&i| self.used[i])
	}
}
