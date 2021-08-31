use crate::storage::Entity;

pub trait QueryFilter {
	fn matches(&self, entity: Entity) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Passthrough;

impl QueryFilter for Passthrough {
	fn matches(&self, _entity: Entity) -> bool {
		true
	}
}

pub struct Not<F>(F);

impl<F> Not<F>
where
	F: QueryFilter,
{
	pub fn new(filter: F) -> Self {
		Self(filter)
	}
}

impl<F> QueryFilter for Not<F>
where
	F: QueryFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		!self.0.matches(entity)
	}
}

pub struct And<F1, F2>(F1, F2);

impl<F1, F2> And<F1, F2>
where
	F1: QueryFilter,
	F2: QueryFilter,
{
	pub fn new(filter1: F1, filter2: F2) -> Self {
		Self(filter1, filter2)
	}
}

impl<F1, F2> QueryFilter for And<F1, F2>
where
	F1: QueryFilter,
	F2: QueryFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) && self.1.matches(entity)
	}
}

pub struct Or<F1, F2>(F1, F2);

impl<F1, F2> Or<F1, F2>
where
	F1: QueryFilter,
	F2: QueryFilter,
{
	pub fn new(filter1: F1, filter2: F2) -> Self {
		Self(filter1, filter2)
	}
}

impl<F1, F2> QueryFilter for Or<F1, F2>
where
	F1: QueryFilter,
	F2: QueryFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) || self.1.matches(entity)
	}
}

macro_rules! impl_filter_ops {
	($ty:ident $(, $filter:ident)*) => {
		impl<$($filter),*> std::ops::Not for $ty<$($filter),*>
		where
			$($filter: QueryFilter,)*
		{
			type Output = Not<Self>;

			fn not(self) -> Self::Output {
				Not::new(self)
			}
		}

		impl<Filter, $($filter),*> std::ops::BitAnd<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = And<Self, Filter>;

			fn bitand(self, filter: Filter) -> Self::Output {
				And::new(self, filter)
			}
		}

		impl<Filter, $($filter),*> std::ops::BitOr<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = Or<Self, Filter>;

			fn bitor(self, filter: Filter) -> Self::Output {
				Or::new(self, filter)
			}
		}
	};
}

impl_filter_ops!(Passthrough);
impl_filter_ops!(Not, F);
impl_filter_ops!(And, F1, F2);
impl_filter_ops!(Or, F1, F2);
