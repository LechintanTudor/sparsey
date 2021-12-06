use crate::components::QueryGroupInfo;
use crate::query::{ChangeTicksFilter, ComponentViewData, GetComponentUnfiltered, Passthrough};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::Ticks;

pub unsafe trait GetComponentSetUnfiltered<'a> {
    const GETS_ONE: bool;

    type Item: 'a;
    type Index: Copy;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn matches_unchecked<F>(&self, index: Self::Index) -> bool
    where
        F: ChangeTicksFilter;

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    unsafe fn get_from_dense_unchecked<F>(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;
}

unsafe impl<'a, G> GetComponentSetUnfiltered<'a> for G
where
    G: GetComponentUnfiltered<'a>,
{
    const GETS_ONE: bool = true;

    type Item = G::Item;
    type Index = usize;
    type Sparse = &'a EntitySparseArray;
    type Data = ComponentViewData<G::Component>;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentUnfiltered::group_info(self).map(QueryGroupInfo::new)
    }

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        info.include(GetComponentUnfiltered::group_info(self)?)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentUnfiltered::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponentUnfiltered::get_index(self, entity)
    }

    unsafe fn matches_unchecked<F>(&self, index: Self::Index) -> bool
    where
        F: ChangeTicksFilter,
    {
        GetComponentUnfiltered::matches_unchecked::<F>(self, index)
    }

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) = GetComponentUnfiltered::get_unchecked::<F>(self, index);
        matches.then(|| item)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        GetComponentUnfiltered::split(self)
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        let (entities, _, data) = GetComponentUnfiltered::split(self);
        (entities, data)
    }

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
        sparse.get(entity)
    }

    unsafe fn get_from_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) =
            G::get_from_parts_unchecked::<F>(*data, index, world_tick, change_tick);

        matches.then(|| item)
    }

    unsafe fn get_from_dense_unchecked<F>(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) =
            G::get_from_parts_unchecked::<F>(*data, index, world_tick, change_tick);

        matches.then(|| item)
    }
}

pub unsafe trait GetComponentSet<'a> {
    const GETS_ONE: bool;

    type Item: 'a;
    type Filter: ChangeTicksFilter;
    type Index: Copy;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn matches_unchecked(&self, index: Self::Index) -> bool;

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item>;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
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

unsafe impl<'a, G> GetComponentSet<'a> for G
where
    G: GetComponentSetUnfiltered<'a>,
{
    const GETS_ONE: bool = G::GETS_ONE;

    type Item = G::Item;
    type Filter = Passthrough;
    type Index = G::Index;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentSetUnfiltered::group_info(self)
    }

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        GetComponentSetUnfiltered::include_group_info(self, info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSetUnfiltered::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponentSetUnfiltered::get_index(self, entity)
    }

    unsafe fn matches_unchecked(&self, index: Self::Index) -> bool {
        GetComponentSetUnfiltered::matches_unchecked::<Self::Filter>(self, index)
    }

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item> {
        GetComponentSetUnfiltered::get_unchecked::<Self::Filter>(self, index)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        GetComponentSetUnfiltered::split_sparse(self)
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        GetComponentSetUnfiltered::split_dense(self)
    }

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
        G::get_index_from_sparse(sparse, entity)
    }

    unsafe fn get_from_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_sparse_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }
}

macro_rules! gets_one {
    ($first:ident) => {
        true
    };
    ($first:ident $(, $other:ident)+) => {
        false
    };
}

macro_rules! replace {
    ($from:tt, $($to:tt) +) => {
        $($to) +
    };
}

macro_rules! new_query_group_info {
    ($first:expr $(, $other:expr)*) => {
        $first.map(QueryGroupInfo::new) $(.and_then(|i| i.include($other?)))*
    };
}

macro_rules! impl_get_component_set {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> GetComponentSetUnfiltered<'a> for ($($elem,)+)
        where
            $($elem: GetComponentUnfiltered<'a>,)+
        {
            const GETS_ONE: bool = gets_one!($($elem),+);

            type Item = ($($elem::Item,)+);
            type Index = ($(replace!($elem, usize),)+);
            type Sparse = ($(replace!($elem, &'a EntitySparseArray),)+);
            type Data = ($(ComponentViewData<$elem::Component>,)+);

            fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
                new_query_group_info!($(self.$idx.group_info()),+)
            }

            #[allow(clippy::needless_question_mark)]
            fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
                Some(info $(.include(self.$idx.group_info()?)?)+)
            }

            fn change_detection_ticks(&self) -> (Ticks, Ticks) {
                self.0.change_detection_ticks()
            }

            fn get_index(&self, entity: Entity) -> Option<Self::Index> {
                Some((
                    $(self.$idx.get_index(entity)?,)+
                ))
            }

            unsafe fn matches_unchecked<Filter>(&self, index: Self::Index) -> bool
            where
                Filter: ChangeTicksFilter,
            {
                if Filter::IS_PASSTHROUGH {
                    true
                } else {
                    $(self.$idx.matches_unchecked::<Filter>(index.$idx))||+
                }
            }

            #[allow(clippy::eval_order_dependence)]
            unsafe fn get_unchecked<Filter>(self, index: Self::Index) -> Option<Self::Item>
            where
                Filter: ChangeTicksFilter,
            {
                if Filter::IS_PASSTHROUGH {
                    Some(($(self.$idx.get_unchecked::<Passthrough>(index.$idx).0,)+))
                } else {
                    let mut matches = false;

                    let item = ($(
                        if !matches {
                            let (item, matched) = self.$idx.get_unchecked::<Filter>(index.$idx);
                            matches |= matched;
                            item
                        } else {
                            self.$idx.get_unchecked::<Passthrough>(index.$idx).0
                        },
                    )+);

                    matches.then(|| item)
                }
            }

            fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
                split_sparse!($((self.$idx, $idx)),+)
            }

            fn split_dense(self) -> (&'a [Entity], Self::Data) {
                split_dense!($((self.$idx, $idx)),+)
            }

            fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
                Some(($(sparse.$idx.get(entity)?,)+))
            }

            #[allow(clippy::eval_order_dependence)]
            unsafe fn get_from_sparse_unchecked<Filter>(
                data: &Self::Data,
                index: Self::Index,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item>
            where
                Filter: ChangeTicksFilter,
            {
                if Filter::IS_PASSTHROUGH {
                    Some(($(
                        $elem::get_from_parts_unchecked::<Passthrough>(
                            data.$idx,
                            index.$idx,
                            world_tick,
                            change_tick,
                        ).0,
                    )+))
                } else {
                    let mut matches = false;

                    let item = ($(
                        if !matches {
                            let (item, matched) = $elem::get_from_parts_unchecked::<Passthrough>(
                                data.$idx,
                                index.$idx,
                                world_tick,
                                change_tick,
                            );

                            matches |= matched;
                            item
                        } else {
                            $elem::get_from_parts_unchecked::<Passthrough>(
                                data.$idx,
                                index.$idx,
                                world_tick,
                                change_tick,
                            ).0
                        },
                    )+);

                    matches.then(|| item)
                }
            }

            #[allow(clippy::eval_order_dependence)]
            unsafe fn get_from_dense_unchecked<Filter>(
                data: &Self::Data,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item>
            where
                Filter: ChangeTicksFilter,
            {
                if Filter::IS_PASSTHROUGH {
                    Some(($(
                        $elem::get_from_parts_unchecked::<Passthrough>(
                            data.$idx,
                            index,
                            world_tick,
                            change_tick,
                        ).0,
                    )+))
                } else {
                    let mut matches = false;

                    let item = ($(
                        if !matches {
                            let (item, matched) = $elem::get_from_parts_unchecked::<Filter>(
                                data.$idx,
                                index,
                                world_tick,
                                change_tick,
                            );

                            matches |= matched;
                            item
                        } else {
                            $elem::get_from_parts_unchecked::<Passthrough>(
                                data.$idx,
                                index,
                                world_tick,
                                change_tick,
                            ).0
                        },
                    )+);

                    matches.then(|| item)
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_get_component_set!((A, 0));
    impl_get_component_set!((A, 0), (B, 1));
    impl_get_component_set!((A, 0), (B, 1), (C, 2));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_get_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
