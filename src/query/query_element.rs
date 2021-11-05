use crate::components::Component;
use crate::group::GroupInfo;
use crate::query::{Contains, QueryElementFilter};
use crate::storage::{Entity, EntitySparseArray, IndexEntity};
use crate::utils::{ChangeTicks, Ticks};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct QueryElementData<'a, T, F> {
    pub components: NonNull<T>,
    pub ticks: NonNull<ChangeTicks>,
    pub filter: F,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T, F> QueryElementData<'a, T, F> {
    pub fn new(components: NonNull<T>, ticks: NonNull<ChangeTicks>, filter: F) -> Self {
        Self {
            components,
            ticks,
            filter,
            _phantom: PhantomData,
        }
    }
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
        NonNull<Self::Component>,
        NonNull<ChangeTicks>,
    );

    unsafe fn get_from_parts_unchecked<F>(
        components: NonNull<Self::Component>,
        ticks: NonNull<ChangeTicks>,
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
        QueryElementData<'a, Self::Component, Self::Filter>,
    );

    unsafe fn get_from_parts_unchecked(
        components: NonNull<Self::Component>,
        ticks: NonNull<ChangeTicks>,
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

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        UnfilteredQueryElement::group_info(self)
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        UnfilteredQueryElement::world_tick(self)
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        UnfilteredQueryElement::change_tick(self)
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        UnfilteredQueryElement::contains(self, entity, &Contains)
    }

    #[inline]
    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        UnfilteredQueryElement::get_index_entity(self, entity)
    }

    #[inline]
    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        UnfilteredQueryElement::get_unchecked(self, index, &Contains)
    }

    #[inline]
    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Component, Self::Filter>,
    ) {
        let (entities, sparse, components, ticks) = UnfilteredQueryElement::split(self);
        (
            entities,
            sparse,
            QueryElementData::new(components, ticks, Contains),
        )
    }

    #[inline]
    unsafe fn get_from_parts_unchecked(
        components: NonNull<Self::Component>,
        ticks: NonNull<ChangeTicks>,
        index: usize,
        filter: &Self::Filter,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        <E as UnfilteredQueryElement>::get_from_parts_unchecked(
            components,
            ticks,
            index,
            filter,
            world_tick,
            change_tick,
        )
    }
}
