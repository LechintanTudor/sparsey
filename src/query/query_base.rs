use crate::group::CombinedGroupInfo;
use crate::query::{
	DenseSplitQueryElement, Include, IncludeExclude, IncludeExcludeFilter, IntoQueryParts,
	IterData, Passthrough, QueryElement, QueryFilter, QueryModifier, SparseSplitQueryElement,
};
use crate::storage::Entity;
use crate::utils::Ticks;

pub unsafe trait QueryBase<'a> {
	type Item;
	type SparseSplit;
	type DenseSplit;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit);

	fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit);

	unsafe fn get_from_sparse_split(
		split: &mut Self::SparseSplit,
		entity: Entity,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<Self::Item>;

	unsafe fn get_from_dense_split(
		split: &mut Self::DenseSplit,
		index: usize,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<Self::Item>;
}

pub trait QueryBaseModifiers<'a>
where
	Self: QueryBase<'a> + Sized,
{
	fn include<I>(self, include: I) -> Include<Self, I>
	where
		I: QueryModifier<'a>,
	{
		Include::new(self, include)
	}

	fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, (), E>
	where
		E: QueryModifier<'a>,
	{
		IncludeExclude::new(self, (), exclude)
	}

	fn filter<F>(self, filter: F) -> IncludeExcludeFilter<Self, (), (), F>
	where
		F: QueryFilter,
	{
		IncludeExcludeFilter::new(self, (), (), filter)
	}
}

impl<'a, B> QueryBaseModifiers<'a> for B
where
	B: QueryBase<'a> + Sized,
{
	// Empty
}

impl<'a, B> IntoQueryParts<'a> for B
where
	B: QueryBase<'a>,
{
	type Base = Self;
	type Include = ();
	type Exclude = ();
	type Filter = Passthrough;

	fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
		(self, (), (), Passthrough)
	}
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

            fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit) {
                split_sparse!($(($elem, self.$idx)),*)
            }

	        fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit) {
                split_dense!($(($elem, self.$idx)),*)
            }

            #[allow(unused_variables)]
            unsafe fn get_from_sparse_split(
                split: &mut Self::SparseSplit,
                entity: Entity,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$elem>(entity, world_tick, change_tick)?,
                )*))
            }

            #[allow(unused_variables)]
            unsafe fn get_from_dense_split(
                split: &mut Self::DenseSplit,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$elem>(index, world_tick, change_tick)?,
                )*))
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
