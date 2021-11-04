use crate::group::GroupInfo;
use crate::query::{QueryElement, QueryElementData, UnfilteredQueryElement};
use crate::storage::{ComponentStorageData, Entity, EntitySparseArray, IndexEntity};
use crate::utils::Ticks;

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

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.0.group_info()
    }

    fn world_tick(&self) -> Ticks {
        self.0.world_tick()
    }

    fn change_tick(&self) -> Ticks {
        self.0.world_tick()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.0.contains(entity)
    }

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.0.get_index_entity(entity)
    }

    unsafe fn get_unchecked(self, index: usize) -> Option<Self::Item> {
        Some(self.0.get_unchecked(index))
    }

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        QueryElementData<'a, Self::Filter>,
    ) {
        self.0.split()
    }

    unsafe fn get_from_parts_unchecked(
        data: &ComponentStorageData,
        index: usize,
        filter: &Self::Filter,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        Some(<E as QueryElement>::get_from_parts_unchecked(
            data,
            index,
            filter,
            world_tick,
            change_tick,
        ))
    }
}
