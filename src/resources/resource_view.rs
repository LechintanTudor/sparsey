use crate::resources::{Resource, ResourceCell};
use crate::utils::Ticks;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// View over a resource of type `T`.
pub struct ResourceView<T, C> {
    cell: C,
    world_tick: Ticks,
    change_tick: Ticks,
    resource: PhantomData<*const T>,
}

impl<T, C> ResourceView<T, C>
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    pub(crate) unsafe fn new(cell: C, world_tick: Ticks, change_tick: Ticks) -> Self {
        Self {
            cell,
            world_tick,
            change_tick,
            resource: PhantomData,
        }
    }
}

impl<T, C> Deref for ResourceView<T, C>
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.cell.resource().downcast_ref::<T>() {
            Some(resource) => resource,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T, C> DerefMut for ResourceView<T, C>
where
    T: Resource,
    C: Deref<Target = ResourceCell> + DerefMut,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell.ticks.tick_mutated = self.world_tick;

        match self.cell.resource_mut().downcast_mut::<T>() {
            Some(resource) => resource,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

/// Returns `true` if the resource was just added.
pub fn res_added<T, C>(resource_view: &ResourceView<T, C>) -> bool
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    resource_view.cell.ticks.tick_added == resource_view.world_tick
}

/// Returns `true` if the resource was mutated.
pub fn res_mutated<T, C>(resource_view: &ResourceView<T, C>) -> bool
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    resource_view.cell.ticks.tick_mutated > resource_view.change_tick
}

/// Returns `true` if the resource was just added or mutated.
pub fn res_changed<T, C>(resource_view: &ResourceView<T, C>) -> bool
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    resource_view.cell.ticks.tick_added == resource_view.world_tick
        || resource_view.cell.ticks.tick_mutated > resource_view.change_tick
}
