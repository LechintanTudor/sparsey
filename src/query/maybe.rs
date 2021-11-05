use std::ptr::NonNull;

use crate::group::GroupInfo;
use crate::query::{QueryElement, QueryElementData, UnfilteredQueryElement};
use crate::storage::{Entity, EntitySparseArray, IndexEntity};
use crate::utils::{ChangeTicks, Ticks};

/// Wrapper over an `UnfilteredQueryElement` which makes it return `Option`s
/// instead of failing.
pub struct Maybe<E>(E);

/// Wrapps the given `QueryElement` in a `Maybe`.
pub fn maybe<'a, E>(element: E) -> Maybe<E>
where
    E: UnfilteredQueryElement<'a>,
{
    Maybe(element)
}

unsafe impl<'a, E> QueryElement<'a> for Maybe<E>
where
    E: QueryElement<'a>,
{
    type Item = Option<E::Item>;
    type Component = E::Component;
    type Filter = E::Filter;

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.0.group_info()
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.0.world_tick()
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.0.world_tick()
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        self.0.contains(entity)
    }

    #[inline]
    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.0.get_index_entity(entity)
    }

    #[inline]
    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        Some(self.0.get_unchecked(index))
    }

    #[inline]
    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Component, Self::Filter>,
    ) {
        self.0.split()
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
        Some(<E as QueryElement>::get_from_parts_unchecked(
            components,
            ticks,
            index,
            filter,
            world_tick,
            change_tick,
        ))
    }
}
