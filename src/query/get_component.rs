use crate::components::{Component, ComponentGroupInfo};
use crate::query::ChangeTicksFilter;
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::{ChangeTicks, Ticks};

pub unsafe trait GetComponent<'a> {
    type Item: 'a;
    type Component: Component;

    fn group_info(&self) -> Option<ComponentGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<usize>;

    unsafe fn matches_unchecked<F>(&self, index: usize) -> bool
    where
        F: ChangeTicksFilter;

    unsafe fn get_unchecked<F>(self, index: usize) -> (Self::Item, bool)
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
    ) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter;
}

pub unsafe trait GetImmutableComponent<'a>
where
    Self: GetComponent<'a>,
{
    fn entities(&self) -> &'a [Entity];

    fn components(&self) -> &'a [Self::Component];
}
