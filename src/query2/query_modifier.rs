use crate::group::CombinedGroupInfo;
use crate::query2::{IterData, UnfilteredImmutableQueryElement};
use crate::storage::{Entity, SparseArrayView};

pub unsafe trait QueryModifier<'a> {
	type Split;

	fn includes(&self, entity: Entity) -> bool;

	fn excludes(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

	fn split(self) -> (Option<IterData<'a>>, Self::Split);

	fn includes_split(split: &Self::Split, entity: Entity) -> bool;

	fn excludes_split(split: &Self::Split, entity: Entity) -> bool;
}

macro_rules! sparse_array_view {
    ($($elem:ident),+) => {
        SparseArrayView<'a>
    };
}

macro_rules! impl_query_modifier {
    ($(($elem:ident, $idx:tt)),*) => {
        unsafe impl<'a, $($elem),*> QueryModifier<'a> for ($($elem,)*)
        where
            $($elem: UnfilteredImmutableQueryElement<'a>,)*
        {
            type Split = ($(sparse_array_view!($elem),)*);

            #[allow(unused_variables)]
            fn includes(&self, entity: Entity) -> bool {
                true $(&& self.$idx.contains(entity))*
            }

            #[allow(unused_variables)]
            fn excludes(&self, entity: Entity) -> bool {
                true $(&& !self.$idx.contains(entity))*
            }

            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)*)
            }

            fn split(self) -> (Option<IterData<'a>>, Self::Split) {
                split_modifier!($(($elem, self.$idx)),*)
            }

            #[allow(unused_variables)]
            fn includes_split(split: &Self::Split, entity: Entity) -> bool {
                true $(&& split.$idx.contains(entity))*
            }

            #[allow(unused_variables)]
            fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
                true $(&& !split.$idx.contains(entity))*
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

    impl_query_modifier!();
	impl_query_modifier!((A, 0));
    impl_query_modifier!((A, 0), (B, 1));
    impl_query_modifier!((A, 0), (B, 1), (C, 2));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}