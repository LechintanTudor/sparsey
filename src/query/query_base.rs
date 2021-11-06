use crate::group::CombinedGroupInfo;
use crate::query::{
    Include, IncludeExclude, IncludeExcludeFilter, IntoQueryParts, IterData, Passthrough,
    QueryElement, QueryElementData, QueryFilter, QueryModifier,
};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::Ticks;

pub unsafe trait QueryBase<'a> {
    const ELEMENT_COUNT: usize;

    type Item: 'a;
    type Sparse;
    type Data;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(&self, entity: Entity) -> bool;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

    unsafe fn get_from_sparse_parts(
        sparse: &Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_from_dense_parts_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;
}

unsafe impl<'a, E> QueryBase<'a> for E
where
    E: QueryElement<'a>,
{
    const ELEMENT_COUNT: usize = 1;

    type Item = E::Item;
    type Sparse = &'a EntitySparseArray;
    type Data = QueryElementData<'a, E::Component, E::Filter>;

    #[inline]
    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = QueryElement::get_index_entity(&self, entity)?.dense();
        unsafe { QueryElement::get_unchecked(self, index) }
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        QueryElement::contains(self, entity)
    }

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        CombinedGroupInfo::default().combine(QueryElement::group_info(self)?)
    }

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data) {
        let world_tick = QueryElement::world_tick(&self);
        let change_tick = QueryElement::change_tick(&self);
        let (entities, sparse, data) = QueryElement::split(self);

        (
            IterData::new(entities, world_tick, change_tick),
            sparse,
            data,
        )
    }

    fn split_dense(self) -> (IterData<'a>, Self::Data) {
        let world_tick = QueryElement::world_tick(&self);
        let change_tick = QueryElement::change_tick(&self);
        let (entities, _, data) = QueryElement::split(self);

        (IterData::new(entities, world_tick, change_tick), data)
    }

    #[inline]
    unsafe fn get_from_sparse_parts(
        sparse: &Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = sparse.get_entity(entity)?.dense();
        E::get_from_parts_unchecked(
            data.components,
            data.ticks,
            index,
            &data.filter,
            world_tick,
            change_tick,
        )
    }

    #[inline]
    unsafe fn get_from_dense_parts_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        E::get_from_parts_unchecked(
            data.components,
            data.ticks,
            index,
            &data.filter,
            world_tick,
            change_tick,
        )
    }
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

macro_rules! entity_sparse_array {
    ($elem:ident) => {
        &'a EntitySparseArray
    };
}

macro_rules! impl_query_base {
    ($count:tt; $(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> QueryBase<'a> for ($($elem,)+)
        where
            $($elem: QueryElement<'a>,)+
        {
            const ELEMENT_COUNT: usize = 2;

            type Item = ($($elem::Item,)+);
            type Sparse = ($(entity_sparse_array!($elem),)+);
            type Data = ($(QueryElementData<'a, $elem::Component, $elem::Filter>,)+);

            fn get(self, entity: Entity) -> Option<Self::Item> {
                let indexes = ($(self.$idx.get_index_entity(entity)?.dense(),)+);

                unsafe {
                    Some((
                        $(self.$idx.get_unchecked(indexes.$idx)?,)+
                    ))
                }
            }


            fn contains(&self, entity: Entity) -> bool {
                $(self.$idx.contains(entity))&&+
            }

            #[allow(clippy::needless_question_mark)]
            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)+)
            }

            fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data) {
                split_sparse!($((self.$idx, $idx)),+)
            }

            fn split_dense(self) -> (IterData<'a>, Self::Data) {
                split_dense!($((self.$idx, $idx)),+)
            }

            #[inline]
            unsafe fn get_from_sparse_parts(
                sparse: &Self::Sparse,
                entity: Entity,
                data: &Self::Data,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                let indexes = ($(sparse.$idx.get_entity(entity)?.dense(),)+);

                Some((
                    $(
                        $elem::get_from_parts_unchecked(
                            data.$idx.components,
                            data.$idx.ticks,
                            indexes.$idx,
                            &data.$idx.filter,
                            world_tick,
                            change_tick,
                        )?,
                    )+
                ))
            }

            #[inline]
            unsafe fn get_from_dense_parts_unchecked(
                data: &Self::Data,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some((
                    $(
                        $elem::get_from_parts_unchecked(
                            data.$idx.components,
                            data.$idx.ticks,
                            index,
                            &data.$idx.filter,
                            world_tick,
                            change_tick,
                        )?,
                    )+
                ))
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_query_base!(1; (A, 0));
    impl_query_base!(2; (A, 0), (B, 1));
    impl_query_base!(3; (A, 0), (B, 1), (C, 2));
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
