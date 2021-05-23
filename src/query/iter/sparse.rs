use crate::query::{
	BaseComponentFilter, BaseQuery, ComponentFilter, IterData, QueryComponentInfoFilter,
};

pub struct SparseIter<'a, Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	data: IterData<'a>,
	index: usize,
	query: Q::SparseSplit,
	include: I::Split,
	exclude: E::Split,
	filter: F,
}

impl<'a, Q, I, E, F> SparseIter<'a, Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	pub fn new(
		data: IterData<'a>,
		query: Q::SparseSplit,
		include: I::Split,
		exclude: E::Split,
		filter: F,
	) -> Self {
		Self {
			data,
			index: 0,
			query,
			include,
			exclude,
			filter,
		}
	}
}

impl<'a, Q, I, E, F> Iterator for SparseIter<'a, Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	type Item = Q::Item;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let entity = *self.data.entities().get(self.index)?;
			self.index += 1;

			if self.filter.matches(entity)
				&& self.include.includes_all(entity)
				&& self.exclude.excludes_all(entity)
			{
				let item = unsafe {
					Q::get_from_sparse_split(
						&mut self.query,
						entity,
						self.data.world_tick(),
						self.data.last_system_tick(),
					)
				};

				if item.is_some() {
					return item;
				}
			}
		}
	}
}