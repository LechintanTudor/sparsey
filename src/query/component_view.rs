use crate::components::{Component, ComponentGroupInfo};
use crate::query::{
    ChangeTicksFilter, ComponentRefMut, GetComponentUnfiltered, GetImmutableComponentUnfiltered,
};
use crate::storage::{ChangeTicks, ComponentStorage, Entity, EntitySparseArray, Ticks};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Container for the data of a `ComponentStorage`. Used internally by queries.
pub struct ComponentViewData<T> {
    /// Pointer to the packed array of components.
    pub components: *mut T,
    /// Pointer to the `ChangeTicks` associated with the components.
    pub ticks: *mut ChangeTicks,
}

impl<T> Clone for ComponentViewData<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ComponentViewData<T> {}

impl<T> ComponentViewData<T> {
    /// Creates a new `ComponentViewData` from the given `components` and `ticks` pointers.
    pub const fn new(components: *mut T, ticks: *mut ChangeTicks) -> Self {
        Self { components, ticks }
    }
}

/// Strongly-typed view over a `ComponentStorage`.
pub struct ComponentView<'a, T, S> {
    storage: S,
    group_info: Option<ComponentGroupInfo<'a>>,
    world_tick: Ticks,
    change_tick: Ticks,
    _phantom: PhantomData<*const T>,
}

impl<'a, T, S> ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    pub(crate) unsafe fn new(
        storage: S,
        group_info: Option<ComponentGroupInfo<'a>>,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self {
        Self { storage, group_info, world_tick, change_tick, _phantom: PhantomData }
    }

    /// Returns the `ChangeTicks` of `entity`'s component.
    #[inline]
    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        self.storage.get_ticks(entity)
    }

    /// Returns the component and `ChangeTicks` of `entity`.
    #[inline]
    pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        unsafe { self.storage.get_with_ticks::<T>(entity) }
    }

    /// Returns the number of components in the view.
    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all entities in the view as a slice.
    #[inline]
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
    }

    /// Returns all components in the view as a slice.
    #[inline]
    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    /// Returns all `ChangeTicks` in the view as a slice.
    #[inline]
    pub fn ticks(&self) -> &[ChangeTicks] {
        self.storage.ticks()
    }
}

impl<'a, T, S> fmt::Debug for ComponentView<'a, T, S>
where
    T: Component + fmt::Debug,
    S: Deref<Target = ComponentStorage>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries = unsafe { self.storage.iter::<T>() };
        f.debug_list().entries(entries).finish()
    }
}

unsafe impl<'a, T, S> GetComponentUnfiltered<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;

    fn group_info(&self) -> Option<ComponentGroupInfo<'a>> {
        self.group_info
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        (self.world_tick, self.change_tick)
    }

    fn get_index(&self, entity: Entity) -> Option<usize> {
        self.storage.get_index(entity)
    }

    unsafe fn matches_unchecked<F>(&self, index: usize) -> bool
    where
        F: ChangeTicksFilter,
    {
        if F::IS_PASSTHROUGH {
            true
        } else {
            let ticks = self.storage.get_ticks_unchecked(index);
            F::matches(ticks, self.world_tick, self.change_tick)
        }
    }

    unsafe fn get_unchecked<F>(self, index: usize) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter,
    {
        if F::IS_PASSTHROUGH {
            (self.storage.get_unchecked(index), true)
        } else {
            let (component, ticks) = self.storage.get_with_ticks_unchecked::<T>(index);
            (component, F::matches(ticks, self.world_tick, self.change_tick))
        }
    }

    fn split(self) -> (&'a [Entity], &'a EntitySparseArray, ComponentViewData<Self::Component>) {
        let (entities, sparse, components, ticks) = self.storage.split();
        (entities, sparse, ComponentViewData::new(components, ticks))
    }

    unsafe fn get_from_parts_unchecked<F>(
        data: ComponentViewData<Self::Component>,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter,
    {
        if F::IS_PASSTHROUGH {
            (&*data.components.add(index), true)
        } else {
            let component = &*data.components.add(index);
            let ticks = &*data.ticks.add(index);
            (component, F::matches(ticks, world_tick, change_tick))
        }
    }
}

unsafe impl<'a, T, S> GetImmutableComponentUnfiltered<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    fn entities(&self) -> &'a [Entity] {
        self.storage.entities()
    }

    fn components(&self) -> &'a [Self::Component] {
        unsafe { self.storage.components::<T>() }
    }
}

unsafe impl<'a, 'b, T, S> GetComponentUnfiltered<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = ComponentRefMut<'a, T>;
    type Component = T;

    fn group_info(&self) -> Option<ComponentGroupInfo<'a>> {
        self.group_info
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        (self.world_tick, self.change_tick)
    }

    fn get_index(&self, entity: Entity) -> Option<usize> {
        self.storage.get_index(entity)
    }

    unsafe fn matches_unchecked<F>(&self, index: usize) -> bool
    where
        F: ChangeTicksFilter,
    {
        if F::IS_PASSTHROUGH {
            true
        } else {
            let ticks = self.storage.get_ticks_unchecked(index);
            F::matches(ticks, self.world_tick, self.change_tick)
        }
    }

    unsafe fn get_unchecked<F>(self, index: usize) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter,
    {
        let (component, ticks) = self.storage.get_with_ticks_unchecked_mut::<T>(index);

        if F::IS_PASSTHROUGH {
            (ComponentRefMut::new(component, ticks, self.world_tick), true)
        } else {
            let matches = F::matches(ticks, self.world_tick, self.change_tick);
            (ComponentRefMut::new(component, ticks, self.world_tick), matches)
        }
    }

    fn split(self) -> (&'a [Entity], &'a EntitySparseArray, ComponentViewData<Self::Component>) {
        let (entities, sparse, components, ticks) = self.storage.split();
        (entities, sparse, ComponentViewData::new(components, ticks))
    }

    unsafe fn get_from_parts_unchecked<F>(
        data: ComponentViewData<Self::Component>,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter,
    {
        let component = &mut *data.components.add(index);
        let ticks = &mut *data.ticks.add(index);

        if F::IS_PASSTHROUGH {
            (ComponentRefMut::new(component, ticks, world_tick), true)
        } else {
            let matches = F::matches(ticks, world_tick, change_tick);
            (ComponentRefMut::new(component, ticks, world_tick), matches)
        }
    }
}
