use crate::query::{ComponentFilter, InfoFilter, IterData, Query};

pub struct SparseIter<'a, Q, I, E, F>
where
	Q: Query<'a>,
	I: ComponentFilter,
	E: ComponentFilter,
	F: InfoFilter,
{
	data: IterData<'a>,
	index: usize,
	query: Q::SparseSplit,
	include: I,
	exclude: E,
	filter: F,
}

impl<'a, Q, I, E, F> Iterator for SparseIter<'a, Q, I, E, F>
where
	Q: Query<'a>,
	I: ComponentFilter,
	E: ComponentFilter,
	F: InfoFilter,
{
	type Item = Q::Item;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let entity = *self.data.entities.get(self.index)?;
			self.index += 1;

			if self.include.includes_all(entity) && self.exclude.excludes_all(entity) {
				let item = unsafe {
					Q::get_from_sparse_split(
						&mut self.query,
						entity,
						self.data.world_tick,
						self.data.last_system_tick,
					)
				};

				if item.is_some() {
					return item;
				}
			}
		}
	}
}
