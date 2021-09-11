use crate::group::GroupInfo;
use crate::query::{Contains, QueryElement, SplitQueryElement, UnfilteredQueryElement};
use crate::storage::Entity;
use crate::utils::{ChangeTicks, Ticks};

/// Wrapper over an `UnfilteredQueryElement` which makes it return `Option`s
/// instead of failing.
#[derive(Clone, Copy, Debug)]
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
    E: UnfilteredQueryElement<'a>,
{
    type Item = Option<E::Item>;
    type Component = E::Component;
    type Filter = Contains;

    #[inline]
    fn get(self, entity: Entity) -> Option<Self::Item> {
        Some(self.0.get(entity))
    }

    #[inline]
    fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
        self.0.get_with_ticks(entity)
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        self.0.contains(entity)
    }

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.0.group_info()
    }

    #[inline]
    fn world_tick(&self) -> crate::Ticks {
        self.0.world_tick()
    }

    #[inline]
    fn change_tick(&self) -> crate::Ticks {
        self.0.change_tick()
    }

    fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
        self.0.split()
    }

    #[inline]
    unsafe fn get_from_parts(
        component: *mut Self::Component,
        ticks: *mut ChangeTicks,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self::Item {
        Some(E::get_from_parts(component, ticks, world_tick, change_tick))
    }
}
