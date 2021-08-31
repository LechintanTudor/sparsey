use crate::group::CombinedGroupInfo;
use crate::query2::{DenseSplitQueryElement, QueryElement, SparseSplitQueryElement};
use crate::storage::Entity;

pub unsafe trait QueryBase<'a> {
	type Item;
	type SparseSplit;
	type DenseSplit;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;
}

macro_rules! impl_query_base {
    ($(($elem:ident, $idx:tt)),*) => {
        unsafe impl<'a, $($elem),*> QueryBase<'a> for ($($elem,)*)
        where
            $($elem: QueryElement<'a>,)*
        {
            type Item = ($($elem::Item,)*);
            type SparseSplit = ($(SparseSplitQueryElement<'a, $elem::Component, $elem::Filter>,)*);
            type DenseSplit = ($(DenseSplitQueryElement<'a, $elem::Component, $elem::Filter>,)*);

            #[allow(unused_variables)]
            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some(($(self.$idx.get(entity)?,)*))
            }

            #[allow(unused_variables)]
            fn contains(&self, entity: Entity) -> bool {
                true $(&& self.$idx.contains(entity))*
            }

            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

    impl_query_base!();
	impl_query_base!((A, 0));
    impl_query_base!((A, 0), (B, 1));
    impl_query_base!((A, 0), (B, 1), (C, 2));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
