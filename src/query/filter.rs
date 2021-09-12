use crate::group::GroupInfo;
use crate::query::{
    And, Not, Or, QueryElement, QueryElementFilter, QueryFilter, SplitQueryElement,
    UnfilteredImmutableQueryElement, UnfilteredQueryElement, Xor,
};
use crate::storage::Entity;
use crate::utils::{ChangeTicks, Ticks};
use std::ops;

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
    /// Creates a new `Filter` with the given `QueryElementFilter` and
    /// `QueryElement`.
    pub fn new(filter: F, element: E) -> Self {
        Self { filter, element }
    }
}

impl<'a, F, E> QueryFilter for Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: UnfilteredImmutableQueryElement<'a>,
{
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

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let (component, ticks) =
            self.element
                .get_with_ticks(entity)
                .map(|(component, ticks)| {
                    (component as *const _ as *mut _, ticks as *const _ as *mut _)
                })?;

        unsafe {
            self.filter
                .matches(
                    &*component,
                    &*ticks,
                    self.element.world_tick(),
                    self.element.change_tick(),
                )
                .then(|| {
                    E::get_from_parts(
                        component,
                        ticks,
                        self.element.world_tick(),
                        self.element.change_tick(),
                    )
                })
        }
    }

    #[inline]
    fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
        self.element.get_with_ticks(entity)
    }

    fn contains(&self, entity: Entity) -> bool {
        self.element
            .get_with_ticks(entity)
            .filter(|(component, ticks)| {
                self.filter.matches(
                    component,
                    ticks,
                    self.element.world_tick(),
                    self.element.change_tick(),
                )
            })
            .is_some()
    }

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.element.group_info()
    }

    #[inline]
    fn world_tick(&self) -> crate::Ticks {
        self.element.world_tick()
    }

    #[inline]
    fn change_tick(&self) -> crate::Ticks {
        self.element.change_tick()
    }

    fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
        let split = self.element.split();
        SplitQueryElement::new(
            split.sparse,
            split.entities,
            split.components,
            split.ticks,
            self.filter,
        )
    }

    #[inline]
    unsafe fn get_from_parts(
        component: *mut Self::Component,
        ticks: *mut ChangeTicks,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self::Item {
        E::get_from_parts(component, ticks, world_tick, change_tick)
    }
}

impl<'a, F, E> ops::Not for Filter<F, E>
where
    F: QueryElementFilter<E::Component>,
    E: UnfilteredImmutableQueryElement<'a>,
{
    type Output = Not<Self>;

    fn not(self) -> Self::Output {
        Not::new(self)
    }
}

impl<'a, F1, E, F2> ops::BitAnd<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: UnfilteredImmutableQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = And<Self, F2>;

    fn bitand(self, filter: F2) -> Self::Output {
        And::new(self, filter)
    }
}

impl<'a, F1, E, F2> ops::BitOr<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: UnfilteredImmutableQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = Or<Self, F2>;

    fn bitor(self, filter: F2) -> Self::Output {
        Or::new(self, filter)
    }
}

impl<'a, F1, E, F2> ops::BitXor<F2> for Filter<F1, E>
where
    F1: QueryElementFilter<E::Component>,
    E: UnfilteredImmutableQueryElement<'a>,
    F2: QueryFilter,
{
    type Output = Xor<Self, F2>;

    fn bitxor(self, filter: F2) -> Self::Output {
        Xor::new(self, filter)
    }
}
