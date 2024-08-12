use crate::entity::{Comp, CompMut, Component, Entity, World};
use crate::query::ViewGroupInfo;

pub unsafe trait QueryElem {
    type View<'a>;
    type Item<'a>;

    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>);

    #[must_use]
    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]>;

    #[must_use]
    fn contains(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    unsafe fn get<'a>(view: &Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>>;

    #[must_use]
    unsafe fn get_by_index<'a>(
        view: &Self::View<'_>,
        entity: Entity,
        index: usize,
    ) -> Self::Item<'a>;
}

unsafe impl QueryElem for Entity {
    type View<'a> = ();
    type Item<'a> = Entity;

    #[inline]
    fn borrow(_world: &World) -> Self::View<'_> {}

    #[inline]
    fn borrow_with_group_info(_world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        ((), None)
    }

    #[inline]
    fn entities<'a>(_view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        None
    }

    #[inline]
    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    unsafe fn get<'a>(_view: &Self::View<'a>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(entity)
    }

    #[inline]
    unsafe fn get_by_index<'a>(
        _view: &Self::View<'_>,
        entity: Entity,
        _index: usize,
    ) -> Self::Item<'a> {
        entity
    }
}

unsafe impl<T> QueryElem for &'_ T
where
    T: Component,
{
    type View<'a> = Comp<'a, T>;
    type Item<'a> = &'a T;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        let (view, info) = world.borrow_with_group_info::<T>();

        let group_info = ViewGroupInfo {
            info,
            len: view.len(),
        };

        (view, Some(group_info))
    }

    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        Some(view.entities())
    }

    fn contains(view: &Self::View<'_>, entity: Entity) -> bool {
        view.contains(entity)
    }

    unsafe fn get<'a>(view: &Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        view.get_ptr(entity).map(|ptr| ptr.as_ref())
    }

    unsafe fn get_by_index<'a>(
        view: &Self::View<'_>,
        _entity: Entity,
        index: usize,
    ) -> Self::Item<'a> {
        view.get_ptr_unchecked(index).as_ref()
    }
}

unsafe impl<T> QueryElem for &'_ mut T
where
    T: Component,
{
    type View<'a> = CompMut<'a, T>;
    type Item<'a> = &'a mut T;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow_mut()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        let (view, info) = world.borrow_with_group_info_mut::<T>();

        let group_info = ViewGroupInfo {
            info,
            len: view.len(),
        };

        (view, Some(group_info))
    }

    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        Some(view.entities())
    }

    fn contains(view: &Self::View<'_>, entity: Entity) -> bool {
        view.contains(entity)
    }

    unsafe fn get<'a>(view: &Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        view.get_ptr(entity).map(|mut ptr| ptr.as_mut())
    }

    unsafe fn get_by_index<'a>(
        view: &Self::View<'_>,
        _entity: Entity,
        index: usize,
    ) -> Self::Item<'a> {
        view.get_ptr_unchecked(index).as_mut()
    }
}

unsafe impl<T> QueryElem for Option<&'_ T>
where
    T: Component,
{
    type View<'a> = Comp<'a, T>;
    type Item<'a> = Option<&'a T>;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow(), None)
    }

    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        Some(view.entities())
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    unsafe fn get<'a>(view: &Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(view.get_ptr(entity).map(|ptr| ptr.as_ref()))
    }

    unsafe fn get_by_index<'a>(
        view: &Self::View<'_>,
        entity: Entity,
        _index: usize,
    ) -> Self::Item<'a> {
        view.get_ptr(entity).map(|ptr| ptr.as_ref())
    }
}

unsafe impl<T> QueryElem for Option<&'_ mut T>
where
    T: Component,
{
    type View<'a> = CompMut<'a, T>;
    type Item<'a> = Option<&'a mut T>;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow_mut()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow_mut(), None)
    }

    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        Some(view.entities())
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    unsafe fn get<'a>(view: &Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(view.get_ptr(entity).map(|mut ptr| ptr.as_mut()))
    }

    unsafe fn get_by_index<'a>(
        view: &Self::View<'_>,
        entity: Entity,
        _index: usize,
    ) -> Self::Item<'a> {
        view.get_ptr(entity).map(|mut ptr| ptr.as_mut())
    }
}
