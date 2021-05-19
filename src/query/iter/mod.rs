mod data;
mod dense;
mod sparse;

pub use self::data::*;
pub use self::dense::*;
pub use self::sparse::*;

use crate::query::{BaseComponentFilter, BaseQuery, QueryComponentInfoFilter};

pub enum Iter<'a, Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	Sparse(SparseIter<'a, Q, I, E, F>),
	Dense(DenseIter<'a, Q, F>),
}

impl<'a, Q, I, E, F> Iterator for Iter<'a, Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	type Item = Q::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Sparse(sparse) => sparse.next(),
			Self::Dense(dense) => dense.next(),
		}
	}

	fn for_each<Func>(self, f: Func)
	where
		Self: Sized,
		Func: FnMut(Self::Item),
	{
		match self {
			Self::Sparse(sparse) => sparse.for_each(f),
			Self::Dense(dense) => dense.for_each(f),
		}
	}
}
