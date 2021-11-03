use crate::components::Component;
use crate::group::GroupInfo;
use crate::query::{
    ComponentRefMut, Contains, ImmutableUnfilteredQueryElement, QueryElement, QueryElementFilter,
    UnfilteredQueryElement,
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

unsafe impl<'a, T, S> UnfilteredQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

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

unsafe impl<'a, T, S> ImmutableUnfilteredQueryElement<'a> for &'a ComponentView<'a, T, S>
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

unsafe impl<'a, 'b, T, S> UnfilteredQueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = ComponentRefMut<'a, T>;
    type Component = T;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

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
