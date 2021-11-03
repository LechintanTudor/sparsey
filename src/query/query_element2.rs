use crate::components::Component;
use crate::query::{Contains, QueryElementFilter};
use crate::storage::{ComponentStorageData, Entity, EntitySparseArray, IndexEntity};
use crate::utils::Ticks;
use crate::GroupInfo;

pub struct QueryElementData<'a, F> {
    pub data: &'a ComponentStorageData,
    pub filter: F,
}

pub unsafe trait UnfilteredQueryElement2<'a> {
    type Item: 'a;
    type Component: Component;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn world_tick(&self) -> Ticks;

    fn change_tick(&self) -> Ticks;

    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>;

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity>;

    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>;

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        &'a ComponentStorageData,
    );

    unsafe fn get_from_parts_unchecked<F>(
        data: &ComponentStorageData,
        index: usize,
        filter: &F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>;
}

pub unsafe trait QueryElement2<'a> {
    type Item: 'a;
    type Component: Component;
    type Filter: QueryElementFilter<Self::Component>;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn world_tick(&self) -> Ticks;

    fn change_tick(&self) -> Ticks;

    fn contains(&self, entity: Entity) -> bool;

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity>;

    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item>;

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Filter>,
    );

    unsafe fn get_from_parts_unchecked(
        data: &ComponentStorageData,
        index: usize,
        filter: &Self::Filter,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;
}

unsafe impl<'a, E> QueryElement2<'a> for E
where
    E: UnfilteredQueryElement2<'a>,
{
    type Item = E::Item;
    type Component = E::Component;
    type Filter = Contains;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        UnfilteredQueryElement2::group_info(self)
    }

    fn world_tick(&self) -> Ticks {
        UnfilteredQueryElement2::world_tick(self)
    }

    fn change_tick(&self) -> Ticks {
        UnfilteredQueryElement2::change_tick(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        UnfilteredQueryElement2::contains(self, entity, &Contains)
    }

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        UnfilteredQueryElement2::get_index_entity(self, entity)
    }

    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        UnfilteredQueryElement2::get_unchecked(self, index, &Contains)
    }

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Filter>,
    ) {
        let (entities, sparse, data) = UnfilteredQueryElement2::split(self);
        (
            entities,
            sparse,
            QueryElementData {
                data,
                filter: Contains,
            },
        )
    }

    unsafe fn get_from_parts_unchecked(
        data: &ComponentStorageData,
        index: usize,
        filter: &Self::Filter,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        <E as UnfilteredQueryElement2>::get_from_parts_unchecked(
            data,
            index,
            filter,
            world_tick,
            change_tick,
        )
    }
}
