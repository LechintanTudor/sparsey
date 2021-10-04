use crate::group::CombinedGroupInfo;
use crate::query::{
    DenseSplitQueryElement, Include, IncludeExclude, IncludeExcludeFilter, IntoQueryParts,
    IterData, Passthrough, QueryElement, QueryFilter, QueryModifier, SparseSplitQueryElement,
};
use crate::storage::Entity;
use crate::utils::Ticks;

/// Trait implemented by the base part of a query. Used for fetching components.
pub unsafe trait QueryBase<'a> {
    const ELEMENT_COUNT: usize;

    type Item;
    type SparseSplit;
    type DenseSplit;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(&self, entity: Entity) -> bool;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn split_sparse(self) -> (IterData<'a>, Self::SparseSplit);

    fn split_dense(self) -> (IterData<'a>, Self::DenseSplit);

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

/// Trait used for applying modifiers to a `QueryBase`.
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

    fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, Passthrough, E>
    where
        E: QueryModifier<'a>,
    {
        IncludeExclude::new(self, Passthrough, exclude)
    }

    fn filter<F>(self, filter: F) -> IncludeExcludeFilter<Self, Passthrough, Passthrough, F>
    where
        F: QueryFilter,
    {
        IncludeExcludeFilter::new(self, Passthrough, Passthrough, filter)
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
    type Include = Passthrough;
    type Exclude = Passthrough;
    type Filter = Passthrough;

    fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
        (self, Passthrough, Passthrough, Passthrough)
    }
}

unsafe impl<'a, E> QueryBase<'a> for E
where
    E: QueryElement<'a>,
{
    const ELEMENT_COUNT: usize = 1;

    type Item = E::Item;
    type SparseSplit = SparseSplitQueryElement<'a, E::Component, E::Filter>;
    type DenseSplit = DenseSplitQueryElement<'a, E::Component, E::Filter>;

    #[inline]
    fn get(self, entity: Entity) -> Option<Self::Item> {
        QueryElement::get(self, entity)
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        QueryElement::contains(self, entity)
    }

    #[inline]
    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        CombinedGroupInfo::default().combine(QueryElement::group_info(self)?)
    }

    fn split_sparse(self) -> (IterData<'a>, Self::SparseSplit) {
        let world_tick = self.world_tick();
        let change_tick = self.change_tick();
        let (entities, sparse_split) = self.split().into_sparse_split();

        (
            IterData::new(entities, world_tick, change_tick),
            sparse_split,
        )
    }

    fn split_dense(self) -> (IterData<'a>, Self::DenseSplit) {
        let world_tick = self.world_tick();
        let change_tick = self.change_tick();
        let (entities, dense_split) = self.split().into_dense_split();

        (
            IterData::new(entities, world_tick, change_tick),
            dense_split,
        )
    }

    #[inline]
    unsafe fn get_from_sparse_split(
        split: &mut Self::SparseSplit,
        entity: Entity,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        split.get::<E>(entity, world_tick, change_tick)
    }

    #[inline]
    unsafe fn get_from_dense_split(
        split: &mut Self::DenseSplit,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        split.get::<E>(index, world_tick, change_tick)
    }
}

macro_rules! impl_query_base {
    ($count:tt; $(#[$attrib:meta];)* $(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> QueryBase<'a> for ($($elem,)+)
        where
            $($elem: QueryElement<'a>,)+
        {
            const ELEMENT_COUNT: usize = $count;

            type Item = ($($elem::Item,)+);
            type SparseSplit = ($(SparseSplitQueryElement<'a, $elem::Component, $elem::Filter>,)+);
            type DenseSplit = ($(DenseSplitQueryElement<'a, $elem::Component, $elem::Filter>,)+);

            $(#[$attrib])*
            #[allow(unused_variables)]
            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some(($(self.$idx.get(entity)?,)+))
            }

            $(#[$attrib])*
            #[allow(unused_variables)]
            fn contains(&self, entity: Entity) -> bool {
                true $(&& self.$idx.contains(entity))+
            }

            #[allow(clippy::needless_question_mark)]
            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)+)
            }

            fn split_sparse(self) -> (IterData<'a>, Self::SparseSplit) {
                split_sparse!($(($elem, self.$idx)),+)
            }

	        fn split_dense(self) -> (IterData<'a>, Self::DenseSplit) {
                split_dense!($(($elem, self.$idx)),+)
            }

            $(#[$attrib])*
            #[allow(unused_variables)]
            unsafe fn get_from_sparse_split(
                split: &mut Self::SparseSplit,
                entity: Entity,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$elem>(entity, world_tick, change_tick)?,
                )+))
            }

            $(#[$attrib])*
            #[allow(unused_variables)]
            unsafe fn get_from_dense_split(
                split: &mut Self::DenseSplit,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$elem>(index, world_tick, change_tick)?,
                )+))
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_query_base!(1; #[inline]; (A, 0));
    impl_query_base!(2; #[inline]; (A, 0), (B, 1));
    impl_query_base!(3; #[inline]; (A, 0), (B, 1), (C, 2));
    impl_query_base!(4; (A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_base!(5; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_base!(6; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_base!(7; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_base!(8; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_base!(9; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_base!(10; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_base!(11; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_base!(12; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_base!(13; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_base!(14; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_base!(15; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_base!(16; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
