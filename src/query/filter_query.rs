use crate::query::{And, Not, Or, Passthrough, Xor};
use crate::storage::Entity;

/// Trait implemented by the part of the `Query` that filters the results.
pub trait QueryFilter {
    /// Returns `true` if the `Entity` macthes the filter.
    fn matches(&self, entity: Entity) -> bool;
}

impl QueryFilter for Passthrough {
    #[inline(always)]
    fn matches(&self, _entity: Entity) -> bool {
        true
    }
}

impl<F> QueryFilter for Not<F>
where
    F: QueryFilter,
{
    #[inline]
    fn matches(&self, entity: Entity) -> bool {
        !self.0.matches(entity)
    }
}

impl<F1, F2> QueryFilter for And<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    #[inline]
    fn matches(&self, entity: Entity) -> bool {
        self.0.matches(entity) && self.1.matches(entity)
    }
}

impl<F1, F2> QueryFilter for Or<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    #[inline]
    fn matches(&self, entity: Entity) -> bool {
        self.0.matches(entity) || self.1.matches(entity)
    }
}

impl<F1, F2> QueryFilter for Xor<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    #[inline]
    fn matches(&self, entity: Entity) -> bool {
        self.0.matches(entity) != self.1.matches(entity)
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
				Not(self)
			}
		}

		impl<Filter, $($filter),*> std::ops::BitAnd<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = And<Self, Filter>;

			fn bitand(self, filter: Filter) -> Self::Output {
				And(self, filter)
			}
		}

		impl<Filter, $($filter),*> std::ops::BitOr<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = Or<Self, Filter>;

			fn bitor(self, filter: Filter) -> Self::Output {
				Or(self, filter)
			}
		}

        impl<Filter, $($filter),*> std::ops::BitXor<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = Xor<Self, Filter>;

			fn bitxor(self, filter: Filter) -> Self::Output {
				Xor(self, filter)
			}
		}
	};
}

impl_filter_ops!(Passthrough);
impl_filter_ops!(Not, F);
impl_filter_ops!(And, F1, F2);
impl_filter_ops!(Or, F1, F2);
