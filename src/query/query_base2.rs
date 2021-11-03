use crate::group::CombinedGroupInfo;
use crate::query::{IterData, QueryElement2, QueryElementData};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::Ticks;

pub unsafe trait QueryBase2<'a> {
    const ELEMENT_COUNT: usize;

    type Item: 'a;
    type Sparse;
    type Data;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(&self, entity: Entity) -> bool;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

    fn get_from_sparse_parts(
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

unsafe impl<'a, E> QueryBase2<'a> for E
where
    E: QueryElement2<'a>,
{
    const ELEMENT_COUNT: usize = 1;

    type Item = E::Item;
    type Sparse = &'a EntitySparseArray;
    type Data = QueryElementData<'a, E::Filter>;

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = QueryElement2::get_index_entity(&self, entity)?.index();
        unsafe { QueryElement2::get_unchecked(self, index) }
    }

    fn contains(&self, entity: Entity) -> bool {
        QueryElement2::contains(self, entity)
    }

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        CombinedGroupInfo::default().combine(QueryElement2::group_info(self)?)
    }

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data) {
        todo!()
    }

    fn split_dense(self) -> (IterData<'a>, Self::Data) {
        todo!()
    }

    fn get_from_sparse_parts(
        sparse: &Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = sparse.get(entity)?.index();

        unsafe {
            E::get_from_parts_unchecked(data.data, index, &data.filter, world_tick, change_tick)
        }
    }

    unsafe fn get_from_dense_parts_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        E::get_from_parts_unchecked(data.data, index, &data.filter, world_tick, change_tick)
    }
}
