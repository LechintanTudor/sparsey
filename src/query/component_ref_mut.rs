use crate::components::Component;
use crate::utils::{ChangeTicks, Ticks};
use std::ops::{Deref, DerefMut};

/// Type returned by mutable queries. Updates the component's `ChangeTicks` when
/// the component is mutated.
pub struct ComponentRefMut<'a, T>
where
    T: Component,
{
    component: &'a mut T,
    ticks: &'a mut ChangeTicks,
    world_tick: Ticks,
}

impl<'a, T> ComponentRefMut<'a, T>
where
    T: Component,
{
    /// Creates a new `ComponentRefMut`which updates the provided `ChangeTicks`
    /// when `deref_mut` is called.
    pub fn new(component: &'a mut T, ticks: &'a mut ChangeTicks, world_tick: Ticks) -> Self {
        Self {
            component,
            ticks,
            world_tick,
        }
    }
}

impl<T> Deref for ComponentRefMut<'_, T>
where
    T: Component,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<T> DerefMut for ComponentRefMut<'_, T>
where
    T: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ticks.tick_mutated = self.world_tick;
        self.component
    }
}
