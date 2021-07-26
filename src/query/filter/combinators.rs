use crate::components::Entity;
use crate::query::{Filter, QueryFilter};

/// Query filter which matches inputs that are not matched by the contained
/// filter.
pub struct Not<F>(F);

impl<F> Not<F> {
	pub(crate) fn new(filter: F) -> Self {
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

/// Query filter which matches inputs matched by both of the contained filters.
pub struct And<F1, F2>(F1, F2);

impl<F1, F2> And<F1, F2> {
	pub(crate) fn new(f1: F1, f2: F2) -> Self {
		Self(f1, f2)
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

/// Query filter which matches inputs matched by either of the contained
/// filters.
pub struct Or<F1, F2>(F1, F2);

impl<F1, F2> Or<F1, F2> {
	pub(crate) fn new(f1: F1, f2: F2) -> Self {
		Self(f1, f2)
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

macro_rules! impl_ops {
	($ty:ident<$($filter:ident),+>, $other_filter:ident) => {
		impl<$($filter),+> std::ops::Not for $ty<$($filter),+> {
			type Output = Not<Self>;

			fn not(self) -> Self::Output {
				Not::new(self)
			}
		}

		impl<$($filter,)+ $other_filter> std::ops::BitAnd<$other_filter> for $ty<$($filter),+>
		where
			$other_filter: QueryFilter,
		{
			type Output = And<Self, $other_filter>;

			fn bitand(self, filter: $other_filter) -> Self::Output {
				And::new(self, filter)
			}
		}

		impl<$($filter,)+ $other_filter> std::ops::BitOr<$other_filter> for $ty<$($filter),+>
		where
			$other_filter: QueryFilter,
		{
			type Output = Or<Self, $other_filter>;

			fn bitor(self, filter: $other_filter) -> Self::Output {
				Or::new(self, filter)
			}
		}
	};
}

impl_ops!(Filter<C, F1>, F2);
impl_ops!(Not<F1>, F2);
impl_ops!(And<F1, F2>, F3);
impl_ops!(Or<F1, F2>, F3);
