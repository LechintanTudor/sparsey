use crate::components::Component;
use crate::group::GroupInfo;
use crate::query::{
    ComponentRefMut, Contains, ImmutableQueryElement, QueryElement, QueryElementFilter,
    SplitQueryElement, UnfilteredImmutableQueryElement, UnfilteredQueryElement2,
};
use crate::storage::{
    ComponentStorage, ComponentStorageData, Entity, EntitySparseArray, IndexEntity,
    TypedComponentStorage,
};
use crate::utils::{ChangeTicks, Ticks};
use std::ops::{Deref, DerefMut};

// TODO: REMOVE TypedComponentStorage

/// View over a `ComponentStorage` of type `T`.
pub struct ComponentView<'a, T, S> {
    pub(crate) storage: TypedComponentStorage<T, S>,
    pub(crate) group_info: Option<GroupInfo<'a>>,
    pub(crate) world_tick: Ticks,
    pub(crate) change_tick: Ticks,
}

impl<'a, T, S> ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    pub(crate) unsafe fn new(
        storage: S,
        group_info: Option<GroupInfo<'a>>,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self {
        Self {
            storage: TypedComponentStorage::new(storage),
            group_info,
            world_tick,
            change_tick,
        }
    }

    /// Returns a wrapper around the view's `ComponentStorage`.
    pub fn storage(&self) -> TypedComponentStorage<T, &ComponentStorage> {
        unsafe { TypedComponentStorage::new(self.storage.storage()) }
    }
}

unsafe impl<'a, T, S> UnfilteredQueryElement2<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;

    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return self.storage.contains(entity);
        }

        let (component, ticks) = match self.storage.get_with_ticks(entity) {
            Some(data) => data,
            None => return false,
        };

        F::matches(filter, component, ticks, self.world_tick, self.change_tick)
    }

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        todo!()
    }

    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        todo!()
    }

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        &'a ComponentStorageData,
    ) {
        todo!()
    }

    unsafe fn get_from_parts_unchecked<F>(
        data: &ComponentStorageData,
        index: usize,
        filter: &F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return Some(&*data.components.cast::<T>().as_ptr().add(index));
        }

        let component = &*data.components.cast::<T>().as_ptr().add(index);
        let ticks = &*data.ticks.as_ptr().add(index);

        if F::matches(filter, component, ticks, world_tick, change_tick) {
            Some(component)
        } else {
            None
        }
    }
}

unsafe impl<'a, T, S> QueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;
    type Filter = Contains;

    #[inline]
    fn get(self, entity: Entity) -> Option<Self::Item> {
        self.storage.get(entity)
    }

    #[inline]
    fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
        self.storage.get_with_ticks(entity)
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

    #[inline]
    fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
        let (sparse, entities, components, ticks) = self.storage.split_for_iteration();
        SplitQueryElement::new(sparse, entities, components as _, ticks as _, Contains)
    }

    #[inline]
    unsafe fn get_from_parts(
        component: *mut Self::Component,
        _ticks: *mut ChangeTicks,
        _world_tick: Ticks,
        _change_tick: Ticks,
    ) -> Self::Item {
        &*component
    }
}

unsafe impl<'a, T, S> ImmutableQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    // Empty
}

unsafe impl<'a, T, S> UnfilteredImmutableQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    #[inline]
    fn entities(&self) -> &'a [Entity] {
        self.storage.entities()
    }

    #[inline]
    fn components(&self) -> &'a [Self::Component] {
        self.storage.components()
    }
}

unsafe impl<'a, 'b, T, S> UnfilteredQueryElement2<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = ComponentRefMut<'a, T>;
    type Component = T;

    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return self.storage.contains(entity);
        }

        let (component, ticks) = match self.storage.get_with_ticks(entity) {
            Some(data) => data,
            None => return false,
        };

        F::matches(filter, component, ticks, self.world_tick, self.change_tick)
    }

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        todo!()
    }

    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        todo!()
    }

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        &'a ComponentStorageData,
    ) {
        todo!()
    }

    unsafe fn get_from_parts_unchecked<F>(
        data: &ComponentStorageData,
        index: usize,
        filter: &F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        let component = &mut *data.components.cast::<T>().as_ptr().add(index);
        let ticks = &mut *data.ticks.as_ptr().add(index);

        if F::IS_PASSTHROUGH {
            return Some(ComponentRefMut::new(component, ticks, world_tick));
        }

        if F::matches(filter, component, ticks, world_tick, change_tick) {
            Some(ComponentRefMut::new(component, ticks, world_tick))
        } else {
            None
        }
    }
}

unsafe impl<'a, 'b, T, S> QueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = ComponentRefMut<'a, T>;
    type Component = T;
    type Filter = Contains;

    #[inline]
    fn get(self, entity: Entity) -> Option<Self::Item> {
        let (component, ticks) = self.storage.get_with_ticks_mut(entity)?;
        Some(ComponentRefMut::new(component, ticks, self.world_tick))
    }

    #[inline]
    fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
        self.storage.get_with_ticks(entity)
    }

    #[inline]
    fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

    #[inline]
    fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
        let (sparse, entities, components, ticks) = self.storage.split_for_iteration_mut();
        SplitQueryElement::new(sparse, entities, components, ticks, Contains)
    }

    #[inline]
    unsafe fn get_from_parts(
        component: *mut Self::Component,
        ticks: *mut ChangeTicks,
        world_tick: Ticks,
        _change_tick: Ticks,
    ) -> Self::Item {
        ComponentRefMut::new(&mut *component, &mut *ticks, world_tick)
    }
}
