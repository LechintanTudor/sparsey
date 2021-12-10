use crate::components::QueryGroupInfo;
use crate::query::{
    query_split_dense, query_split_sparse, ComponentViewData, GetComponentSet,
    GetComponentUnfiltered, IntoQueryParts, Passthrough,
};
use crate::storage::{Entity, EntitySparseArray, Ticks};
use crate::utils::impl_generic_tuple_1_16;

/// Trait used to fetch filtered component sets from component views. Used internally by queries.
pub unsafe trait QueryGet<'a> {
    /// Whether or not a single component is fetched. Used internally by queries for optimization
    /// purposes.
    const GETS_ONE: bool;

    /// Fetched item.
    type Item: 'a;
    /// `EntitySparseArray`s returned when splitting the views.
    type Sparse: 'a;
    /// `ComponentStorageData` returned when splitting the views.
    type Data: 'a;

    /// Returns the group to which the storages belong, if any.
    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    /// Returns the world tick and change tick used for change detection.
    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    /// Returns `true` if `entity` matches the filters.
    fn contains(&self, entity: Entity) -> bool;

    /// Returns the item if `entity` matches the filters.
    fn get(self, entity: Entity) -> Option<Self::Item>;

    /// Splits the views for sparse iteration.
    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    /// Splits the views for dense iteration.
    fn split_dense(self) -> (&'a [Entity], Self::Data);

    /// Returns the item if `entity` matches the filters. Used internally by `SparseIter`.
    unsafe fn get_from_sparse_unchecked(
        sparse: &Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    /// Returns the item if the data at `index` matches the filters. Used internally by `DenseIter`.
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
    const GETS_ONE: bool = true;

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
        sparse: &Self::Sparse,
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

macro_rules! gets_one {
    ($first:ident) => {
        $first::GETS_ONE
    };
    ($first:ident $(, $other:ident)+) => {
        false
    };
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
            const GETS_ONE: bool = gets_one!($($elem),+);

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
                sparse: &Self::Sparse,
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

impl_generic_tuple_1_16!(impl_query_get);
