use crate::resources::{Resource, ResourceCell};
use crate::utils::{Ticks, UnsafeUnwrap};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// View over a resource of type `T`.
pub struct ResourceView<T, C> {
    cell: C,
    world_tick: Ticks,
    change_tick: Ticks,
    _phantom: PhantomData<*const T>,
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
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Deref for ResourceView<T, C>
where
    T: Resource,
    C: Deref<Target = ResourceCell>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.cell.value().downcast_ref::<T>().unsafe_unwrap() }
    }
}

impl<T, C> DerefMut for ResourceView<T, C>
where
    T: Resource,
    C: Deref<Target = ResourceCell> + DerefMut,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell.ticks.tick_mutated = self.world_tick;
        unsafe { self.cell.value_mut().downcast_mut::<T>().unsafe_unwrap() }
    }
}

impl<T, C> fmt::Debug for ResourceView<T, C>
where
    T: Resource + fmt::Debug,
    C: Deref<Target = ResourceCell>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResourceView")
            .field("value", self.deref())
            .field("ticks", self.cell.ticks())
            .finish()
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
