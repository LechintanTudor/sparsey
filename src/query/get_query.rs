use crate::components::QueryGroupInfo;
use crate::query::{
    ComponentViewData, GetComponentSet, GetComponentUnfiltered, IntoQueryParts, Passthrough,
};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::Ticks;

pub unsafe trait QueryGet<'a> {
    type Item: 'a;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn contains(&self, entity: Entity) -> bool;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    unsafe fn get_from_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;
}

impl<'a, G> IntoQueryParts<'a> for G
where
    G: QueryGet<'a>,
{
    type Get = G;
    type Include = Passthrough;
    type Exclude = Passthrough;
    type Filter = Passthrough;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude, Self::Filter) {
        (self, Passthrough, Passthrough, Passthrough)
    }
}

unsafe impl<'a, G> QueryGet<'a> for G
where
    G: GetComponentUnfiltered<'a>,
{
    type Item = G::Item;
    type Sparse = &'a EntitySparseArray;
    type Data = ComponentViewData<G::Component>;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentUnfiltered::group_info(self).map(QueryGroupInfo::new)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentUnfiltered::change_detection_ticks(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        let index = match GetComponentUnfiltered::get_index(self, entity) {
            Some(index) => index,
            None => return false,
        };

        unsafe { GetComponentUnfiltered::matches_unchecked::<Passthrough>(self, index) }
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = GetComponentUnfiltered::get_index(&self, entity)?;
        let (item, matches) =
            unsafe { GetComponentUnfiltered::get_unchecked::<Passthrough>(self, index) };

        matches.then(|| item)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        GetComponentUnfiltered::split(self)
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        let (entities, _, data) = GetComponentUnfiltered::split(self);
        (entities, data)
    }

    unsafe fn get_from_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = sparse.get(entity)?;
        let (item, matches) =
            G::get_from_parts_unchecked::<Passthrough>(*data, index, world_tick, change_tick);

        matches.then(|| item)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let (item, matches) =
            G::get_from_parts_unchecked::<Passthrough>(*data, index, world_tick, change_tick);

        matches.then(|| item)
    }
}

macro_rules! new_group_info {
    ($first:expr $(, $other:expr)*) => {
        $first.group_info() $(.and_then(|i| $other.include_group_info(i)))*
    };
}

macro_rules! impl_query_get {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> QueryGet<'a> for ($($elem,)+)
        where
            $($elem: GetComponentSet<'a>,)+
        {
            type Item = ($($elem::Item,)+);
            type Sparse = ($($elem::Sparse,)+);
            type Data = ($($elem::Data,)+);

            fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
                new_group_info!($(self.$idx),+)
            }

            fn change_detection_ticks(&self) -> (Ticks, Ticks) {
                self.0.change_detection_ticks()
            }

            fn contains(&self, entity: Entity) -> bool {
                let index = ($(
                    match self.$idx.get_index(entity) {
                        Some(index) => index,
                        None => return false,
                    },
                )+);

                unsafe {
                    $(self.$idx.matches_unchecked(index.$idx))&&+
                }
            }

            fn get(self, entity: Entity) -> Option<Self::Item> {
                let index = ($(self.$idx.get_index(entity)?,)+);

                unsafe {
                    Some(($(self.$idx.get_unchecked(index.$idx)?,)+))
                }
            }

            fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
                query_split_sparse!($((self.$idx, $idx)),+)
            }

            fn split_dense(self) -> (&'a [Entity], Self::Data) {
                query_split_dense!($((self.$idx, $idx)),+)
            }

            unsafe fn get_from_sparse_unchecked(
                sparse: &'a Self::Sparse,
                entity: Entity,
                data: &Self::Data,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                let index = ($(
                    $elem::get_index_from_sparse(&sparse.$idx, entity)?,
                )+);

                Some(($(
                    $elem::get_from_sparse_unchecked(&data.$idx, index.$idx, world_tick, change_tick)?,
                )+))
            }

            unsafe fn get_from_dense_unchecked(
                data: &Self::Data,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    $elem::get_from_dense_unchecked(&data.$idx, index, world_tick, change_tick)?,
                )+))
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query_get!((A, 0));
    impl_query_get!((A, 0), (B, 1));
    impl_query_get!((A, 0), (B, 1), (C, 2));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
