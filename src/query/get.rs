use crate::components::QueryGroupInfo;
use crate::query::{GetComponentSet, IntoQueryParts, IterData, Passthrough};
use crate::storage::Entity;
use crate::utils::Ticks;

pub unsafe trait QueryGet<'a> {
    type Item: 'a;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn contains(&self, entity: Entity) -> bool;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

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

unsafe impl<'a, G> QueryGet<'a> for G
where
    G: GetComponentSet<'a>,
{
    type Item = G::Item;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentSet::group_info(self)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSet::change_detection_ticks(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        let index = match GetComponentSet::get_index(self, entity) {
            Some(index) => index,
            None => return false,
        };

        unsafe { GetComponentSet::matches_unchecked(self, index) }
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = GetComponentSet::get_index(&self, entity)?;
        unsafe { GetComponentSet::get_unchecked(self, index) }
    }

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data) {
        let (world_tick, change_tick) = GetComponentSet::change_detection_ticks(&self);
        let (entities, sparse, data) = GetComponentSet::split_sparse(self);

        (
            IterData::new(entities, world_tick, change_tick),
            sparse,
            data,
        )
    }

    fn split_dense(self) -> (IterData<'a>, Self::Data) {
        let (world_tick, change_tick) = GetComponentSet::change_detection_ticks(&self);
        let (entities, _, data) = GetComponentSet::split_sparse(self);

        (IterData::new(entities, world_tick, change_tick), data)
    }

    unsafe fn get_from_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = G::get_index_from_sparse(sparse, entity)?;
        G::get_from_sparse_unchecked(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked(data, index, world_tick, change_tick)
    }
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
