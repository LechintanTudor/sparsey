use crate::components::QueryGroupInfo;
use crate::query::{ChangeTicksFilter, GetComponentSet, GetComponentSetUnfiltered};
use crate::storage::Entity;
use crate::utils::Ticks;
use std::marker::PhantomData;

pub struct Filter<F, G> {
    get: G,
    _phantom: PhantomData<F>,
}

impl<'a, F, G> Filter<F, G>
where
    F: ChangeTicksFilter,
    G: GetComponentSetUnfiltered<'a>,
{
    pub fn new(get: G) -> Self {
        Self {
            get,
            _phantom: PhantomData,
        }
    }
}

unsafe impl<'a, F, G> GetComponentSet<'a> for Filter<F, G>
where
    F: ChangeTicksFilter,
    G: GetComponentSetUnfiltered<'a>,
{
    type Item = G::Item;
    type Filter = F;
    type Index = G::Index;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        self.get.include_group_info(info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        self.get.change_detection_ticks()
    }

    fn contains(&self, entity: Entity) -> bool
    where
        F: ChangeTicksFilter,
    {
        self.get.contains::<F>(entity)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        self.get.get_index(entity)
    }

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item> {
        self.get.get_unchecked::<F>(index)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        self.get.split_sparse()
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        self.get.split_dense()
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
        G::get_from_sparse_unchecked::<F>(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked::<F>(data, index, world_tick, change_tick)
    }
}
