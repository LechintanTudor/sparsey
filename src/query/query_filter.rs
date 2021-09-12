use crate::storage::Entity;

/// Trait implemented by query filters.
pub trait QueryFilter {
    fn matches(&self, entity: Entity) -> bool;
}

/// Filter that matches all entities.
#[derive(Clone, Copy, Debug)]
pub struct Passthrough;

impl QueryFilter for Passthrough {
    fn matches(&self, _entity: Entity) -> bool {
        true
    }
}

/// Wrapper around a `QueryFilter` which negates its result.
pub struct Not<F>(F);

impl<F> Not<F>
where
    F: QueryFilter,
{
    /// Creates a new `Not` with the given filter.
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

/// `QueryFilter` that only matches entities which match both the filters
/// contained inside.
pub struct And<F1, F2>(F1, F2);

impl<F1, F2> And<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    /// Creates a new `And` with the given filters.
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

/// `QueryFilter` that only matches entities which match either of the filters
/// contained inside.
pub struct Or<F1, F2>(F1, F2);

impl<F1, F2> Or<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    /// Creates a new `Or` with the given filters.
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

/// `QueryFilter` that only matches entities which match only one of the filters
/// contained inside.
pub struct Xor<F1, F2>(F1, F2);

impl<F1, F2> Xor<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    /// Creates a new `Xor` with the given filters.
    pub fn new(filter1: F1, filter2: F2) -> Self {
        Self(filter1, filter2)
    }
}

impl<F1, F2> QueryFilter for Xor<F1, F2>
where
    F1: QueryFilter,
    F2: QueryFilter,
{
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

        impl<Filter, $($filter),*> std::ops::BitXor<Filter> for $ty<$($filter),*>
		where
			Filter: QueryFilter,
			$($filter: QueryFilter,)*
		{
			type Output = Xor<Self, Filter>;

			fn bitxor(self, filter: Filter) -> Self::Output {
				Xor::new(self, filter)
			}
		}
	};
}

impl_filter_ops!(Passthrough);
impl_filter_ops!(Not, F);
impl_filter_ops!(And, F1, F2);
impl_filter_ops!(Or, F1, F2);
