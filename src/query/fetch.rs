use crate::components::{CombinedGroupInfo, Component, GroupInfo};
use crate::query::IterData;
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::{ChangeTicks, Ticks};

pub trait ChangeTicksFilter
where
    Self: 'static,
{
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, change_tick: Ticks) -> bool;
}

pub trait FetchComponent<'a> {
    type Item: 'a;
    type Component: Component;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn contains<F>(&self, entity: Entity) -> bool
    where
        F: ChangeTicksFilter;

    fn get_index(&self, entity: Entity) -> Option<usize>;

    unsafe fn get_unchecked<F>(self, index: usize) -> Option<Self::Item>;

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

pub trait FetchComponentSet<'a> {
    type Item: 'a;
    type Index: Copy;
    type Sparse;
    type Data;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

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

pub trait FetchComponentSetFiltered<'a> {
    type Item: 'a;
    type Filter: ChangeTicksFilter;
    type Index: Copy;
    type Sparse;
    type Data;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

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

pub trait QueryFetch<'a> {
    type Item;
    type Sparse;
    type Data;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(&self, entity: Entity) -> bool;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

    unsafe fn get_sparse_unchecked(
        sparse: &'a Self::Sparse,
        index: usize,
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
