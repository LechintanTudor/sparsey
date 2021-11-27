use crate::components::{Component, QueryGroupInfo};
use crate::query::{ChangeTicksFilter, IterData, Passthrough};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::{ChangeTicks, Ticks};

#[derive(Clone, Copy)]
pub struct ComponentViewData<T> {
    pub components: *mut T,
    pub ticks: *mut ChangeTicks,
}

impl<T> ComponentViewData<T> {
    pub const fn new(components: *mut T, ticks: *mut ChangeTicks) -> Self {
        Self { components, ticks }
    }
}

pub unsafe trait GetComponent<'a> {
    type Item: 'a;
    type Component: Component;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<usize>;

    unsafe fn get_unchecked<F>(self, index: usize) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        *mut Self::Component,
        *mut ChangeTicks,
    );

    unsafe fn get_from_parts_unchecked<F>(
        components: *mut Self::Component,
        ticks: *mut ChangeTicks,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;
}

pub unsafe trait GetComponentSetUnfiltered<'a> {
    type Item: 'a;
    type Index: Copy;
    type Sparse: 'a;
    type Data;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    unsafe fn get_dense_unchecked<F>(
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
    G: GetComponent<'a>,
{
    type Item = G::Item;
    type Index = usize;
    type Sparse = &'a EntitySparseArray;
    type Data = ComponentViewData<G::Component>;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        GetComponent::include_group_info(self, info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponent::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponent::get_index(self, entity)
    }

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        GetComponent::get_unchecked::<F>(self, index)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        let (entities, sparse, components, ticks) = GetComponent::split(self);
        (entities, sparse, ComponentViewData::new(components, ticks))
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        let (entities, _, components, ticks) = GetComponent::split(self);
        (entities, ComponentViewData::new(components, ticks))
    }

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
        sparse.get_entity(entity).map(|e| e.dense())
    }

    unsafe fn get_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        G::get_from_parts_unchecked::<F>(
            data.components,
            data.ticks,
            index,
            world_tick,
            change_tick,
        )
    }

    unsafe fn get_dense_unchecked<F>(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        G::get_from_parts_unchecked::<F>(
            data.components,
            data.ticks,
            index,
            world_tick,
            change_tick,
        )
    }
}

pub unsafe trait GetComponentSet<'a> {
    type Item: 'a;
    type Filter: ChangeTicksFilter;
    type Index: Copy;
    type Sparse;
    type Data;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item>;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_dense_unchecked(
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
    type Item = G::Item;
    type Filter = Passthrough;
    type Index = G::Index;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        GetComponentSetUnfiltered::include_group_info(self, info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSetUnfiltered::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponentSetUnfiltered::get_index(self, entity)
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

    unsafe fn get_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_sparse_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }

    unsafe fn get_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_dense_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }
}

pub unsafe trait QueryGet<'a> {
    type Item;
    type Sparse;
    type Data;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

    unsafe fn get_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_dense_unchecked(
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

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        GetComponentSet::include_group_info(self, info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSet::change_detection_ticks(self)
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

    unsafe fn get_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = G::get_index_from_sparse(sparse, entity)?;
        G::get_sparse_unchecked(data, index, world_tick, change_tick)
    }

    unsafe fn get_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_dense_unchecked(data, index, world_tick, change_tick)
    }
}
