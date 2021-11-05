use crate::group::GroupInfo;
use crate::query::{
    And, ImmutableUnfilteredQueryElement, Not, Or, QueryElement, QueryElementData,
    QueryElementFilter, QueryFilter, UnfilteredQueryElement, Xor,
};
use crate::storage::{Entity, EntitySparseArray, IndexEntity};
use crate::utils::{ChangeTicks, Ticks};
use std::ops;
use std::ptr::NonNull;

/// Wrapper around a `QueryElement`. Used for applying filters.
pub struct Filter<F, E> {
    filter: F,
    element: E,
}

impl<'a, F, E> Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: UnfilteredQueryElement<'a>,
{
    /// Applies a filter to the given `QueryElement`.
    pub fn new(element: E, filter: F) -> Self {
        Self { element, filter }
    }
}

impl<'a, F, E> QueryFilter for Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: ImmutableUnfilteredQueryElement<'a>,
{
    #[inline]
    fn matches(&self, entity: Entity) -> bool {
        <Self as QueryElement<'a>>::contains(self, entity)
    }
}

unsafe impl<'a, F, E> QueryElement<'a> for Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: UnfilteredQueryElement<'a>,
{
    type Item = E::Item;
    type Component = E::Component;
    type Filter = F;

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.element.group_info()
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.element.world_tick()
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.element.change_tick()
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        self.element.contains(entity, &self.filter)
    }

    #[inline]
    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.element.get_index_entity(entity)
    }

    #[inline]
    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        self.element.get_unchecked(index, &self.filter)
    }

    #[inline]
    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Component, Self::Filter>,
    ) {
        let (entities, sparse, components, ticks) = E::split(self.element);
        (
            entities,
            sparse,
            QueryElementData::new(components, ticks, self.filter),
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
        E::get_from_parts_unchecked(components, ticks, index, filter, world_tick, change_tick)
    }
}

impl<'a, F, E> ops::Not for Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: UnfilteredQueryElement<'a>,
{
    type Output = Filter<Not<F>, E>;

    fn not(self) -> Self::Output {
        Filter::new(self.element, Not(self.filter))
    }
}

impl<'a, F1, E, F2> ops::BitAnd<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: ImmutableUnfilteredQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = And<Self, F2>;

    fn bitand(self, filter: F2) -> Self::Output {
        And(self, filter)
    }
}

impl<'a, F1, E, F2> ops::BitOr<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: ImmutableUnfilteredQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = Or<Self, F2>;

    fn bitor(self, filter: F2) -> Self::Output {
        Or(self, filter)
    }
}

impl<'a, F1, E, F2> ops::BitXor<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: ImmutableUnfilteredQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = Xor<Self, F2>;

    fn bitxor(self, filter: F2) -> Self::Output {
        Xor(self, filter)
    }
}
