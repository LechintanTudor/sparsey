pub use self::added::*;
pub use self::combinators::*;

use crate::components::Entity;

mod added;
mod combinators;

pub trait StateFilter {
	fn matches(&self, entity: Entity) -> bool;
}

macro_rules! impl_state_filter_combinators {
    ($ty:ident $(, $generic:ident)+) => {
        impl<Filter, $($generic),+> std::ops::BitAnd<Filter> for $ty<$($generic),+>
        where
            Filter: $crate::query::StateFilter,
        {
            type Output = $crate::query::AndStateFilter<Self, Filter>;

            fn bitand(self, state: Filter) -> Self::Output {
                $crate::query::AndStateFilter::new(self, state)
            }
        }

        impl<Filter, $($generic),+> std::ops::BitOr<Filter> for $ty<$($generic),+>
        where
            Filter: $crate::query::StateFilter,
        {
            type Output = $crate::query::OrStateFilter<Self, Filter>;

            fn bitor(self, state: Filter) -> Self::Output {
                $crate::query::OrStateFilter::new(self, state)
            }
        }
    };
}

impl_state_filter_combinators!(AndStateFilter, F1, F2);
impl_state_filter_combinators!(OrStateFilter, F1, F2);
impl_state_filter_combinators!(Added, Q);
impl_state_filter_combinators!(NotAdded, Q);
