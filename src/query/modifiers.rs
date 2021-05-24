use crate::query::{QueryComponentFilter, QueryComponentInfoFilter};

pub struct Include<Q, I> {
	pub(super) query: Q,
	pub(super) include: I,
}

impl<Q, I> Include<Q, I> {
	pub(super) fn new(query: Q, include: I) -> Self {
		Self { query, include }
	}

	pub fn exclude<'a, E>(self, exclude: E) -> IncludeExclude<Q, I, E>
	where
		E: QueryComponentFilter<'a>,
	{
		IncludeExclude::new(self.query, self.include, exclude)
	}

	pub fn filter<'a, F>(self, filter: F) -> IncludeExcludeFilter<Q, I, (), F>
	where
		F: QueryComponentFilter<'a>,
	{
		IncludeExcludeFilter::new(self.query, self.include, (), filter)
	}
}

pub struct IncludeExclude<Q, I, E> {
	pub(super) query: Q,
	pub(super) include: I,
	pub(super) exclude: E,
}

impl<Q, I, E> IncludeExclude<Q, I, E> {
	pub(super) fn new(query: Q, include: I, exclude: E) -> Self {
		Self {
			query,
			include,
			exclude,
		}
	}

	pub fn filter<'a, F>(self, filter: F) -> IncludeExcludeFilter<Q, I, E, F>
	where
		F: QueryComponentInfoFilter,
	{
		IncludeExcludeFilter::new(self.query, self.include, self.exclude, filter)
	}
}

pub struct IncludeExcludeFilter<Q, I, E, F> {
	pub(super) query: Q,
	pub(super) include: I,
	pub(super) exclude: E,
	pub(super) filter: F,
}

impl<Q, I, E, F> IncludeExcludeFilter<Q, I, E, F> {
	pub(super) fn new(query: Q, include: I, exclude: E, filter: F) -> Self {
		Self {
			query,
			include,
			exclude,
			filter,
		}
	}
}
