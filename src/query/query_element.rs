use crate::components::Component;
use crate::query::{Contains, QueryElementFilter};
use crate::storage::{ComponentStorageData, Entity, EntitySparseArray, IndexEntity};
use crate::utils::Ticks;
use crate::GroupInfo;

pub struct QueryElementData<'a, F> {
    pub data: &'a ComponentStorageData,
    pub filter: F,
}

pub unsafe trait UnfilteredQueryElement<'a> {
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

pub unsafe trait ImmutableUnfilteredQueryElement<'a>
where
    Self: UnfilteredQueryElement<'a>,
{
    fn entities(&self) -> &'a [Entity];

    fn components(&self) -> &'a [Self::Component];
}

pub unsafe trait QueryElement<'a> {
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

unsafe impl<'a, E> QueryElement<'a> for E
where
    E: UnfilteredQueryElement<'a>,
{
    type Item = E::Item;
    type Component = E::Component;
    type Filter = Contains;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        UnfilteredQueryElement::group_info(self)
    }

    fn world_tick(&self) -> Ticks {
        UnfilteredQueryElement::world_tick(self)
    }

    fn change_tick(&self) -> Ticks {
        UnfilteredQueryElement::change_tick(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        UnfilteredQueryElement::contains(self, entity, &Contains)
    }

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        UnfilteredQueryElement::get_index_entity(self, entity)
    }

    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        UnfilteredQueryElement::get_unchecked(self, index, &Contains)
    }

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Filter>,
    ) {
        let (entities, sparse, data) = UnfilteredQueryElement::split(self);
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
        <E as UnfilteredQueryElement>::get_from_parts_unchecked(
            data,
            index,
            filter,
            world_tick,
            change_tick,
        )
    }
}
