use crate::component::{Component, View, ViewGroupInfo, ViewMut};
use crate::entity::{Entity, SparseVec};
use crate::World;
use core::ops::Range;
use core::ptr::NonNull;
use core::slice;

pub unsafe trait QueryElem {
    type View<'a>;
    type Item<'a>;
    type Slice<'a>;
    type Sparse<'a>: Copy;
    type SparseParts<'a>: Copy;
    type DenseParts<'a>: Copy;

    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>);

    #[must_use]
    fn contains(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>>;

    #[must_use]
    fn split_sparse<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>);

    #[must_use]
    fn sparse_contains(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>);

    #[must_use]
    unsafe fn get_sparse<'a>(part: Self::SparseParts<'a>, entity: Entity)
        -> Option<Self::Item<'a>>;

    #[must_use]
    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>);

    #[must_use]
    unsafe fn get_dense<'a>(
        parts: Self::DenseParts<'a>,
        index: usize,
        entity: Entity,
    ) -> Self::Item<'a>;

    #[must_use]
    unsafe fn slice<'a>(
        parts: Self::DenseParts<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a>;
}

unsafe impl QueryElem for Entity {
    type View<'a> = ();
    type Item<'a> = Entity;
    type Slice<'a> = &'a [Entity];
    type Sparse<'a> = ();
    type SparseParts<'a> = ();
    type DenseParts<'a> = ();

    #[inline]
    fn borrow(_world: &World) -> Self::View<'_> {
        ()
    }

    #[inline]
    fn borrow_with_group_info(_world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        ((), None)
    }

    #[inline]
    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    fn get<'a>(_view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(entity)
    }

    #[inline]
    fn split_sparse<'a>(_view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    #[inline]
    fn sparse_contains(_sparse: Self::Sparse<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    fn split_sparse_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        (None, ())
    }

    #[inline]
    unsafe fn get_sparse<'a>(
        _part: Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        Some(entity)
    }

    #[inline]
    fn split_dense_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        (None, ())
    }

    #[inline]
    unsafe fn get_dense<'a>(
        _part: Self::DenseParts<'a>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'a> {
        entity
    }

    #[inline]
    unsafe fn slice<'a>(
        _parts: Self::DenseParts<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        entities.get_unchecked(range)
    }
}

unsafe impl<T> QueryElem for &'_ T
where
    T: Component,
{
    type View<'a> = View<'a, T>;
    type Item<'a> = &'a T;
    type Slice<'a> = &'a [T];
    type Sparse<'a> = &'a SparseVec;
    type SparseParts<'a> = (&'a SparseVec, NonNull<T>);
    type DenseParts<'a> = NonNull<T>;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        let (view, info) = world.borrow_with_group_info::<T>();

        let info = ViewGroupInfo {
            info,
            len: view.len(),
        };

        (view, Some(info))
    }

    fn contains(view: &Self::View<'_>, entity: Entity) -> bool {
        view.contains(entity)
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        view.get(entity)
    }

    fn split_sparse<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (Some(view.entities()), view.sparse())
    }

    fn sparse_contains(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        (
            Some(view.entities()),
            (view.sparse(), view.as_non_null_ptr()),
        )
    }

    unsafe fn get_sparse<'a>(
        (sparse, ptr): Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        let i = sparse.get_sparse(entity.sparse())?.dense();
        Some(ptr.add(i).as_ref())
    }

    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        (Some(view.entities()), view.as_non_null_ptr())
    }

    unsafe fn get_dense<'a>(
        ptr: Self::DenseParts<'a>,
        index: usize,
        _entity: Entity,
    ) -> Self::Item<'a> {
        ptr.add(index).as_ref()
    }

    unsafe fn slice<'a>(
        ptr: Self::DenseParts<'a>,
        _entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        slice::from_raw_parts(ptr.as_ptr(), range.end - range.start)
    }
}

unsafe impl<T> QueryElem for &'_ mut T
where
    T: Component,
{
    type View<'a> = ViewMut<'a, T>;
    type Item<'a> = &'a mut T;
    type Slice<'a> = &'a mut [T];
    type Sparse<'a> = &'a SparseVec;
    type SparseParts<'a> = (&'a SparseVec, NonNull<T>);
    type DenseParts<'a> = NonNull<T>;

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow_mut::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        let (view, info) = world.borrow_with_group_info_mut::<T>();

        let info = ViewGroupInfo {
            info,
            len: view.len(),
        };

        (view, Some(info))
    }

    fn contains(view: &Self::View<'_>, entity: Entity) -> bool {
        view.contains(entity)
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        view.get_mut(entity)
    }

    fn split_sparse<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (Some(view.entities()), view.sparse())
    }

    fn sparse_contains(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        (
            Some(view.entities()),
            (view.sparse(), view.as_non_null_ptr()),
        )
    }

    unsafe fn get_sparse<'a>(
        (sparse, ptr): Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        let i = sparse.get_sparse(entity.sparse())?.dense();
        Some(ptr.add(i).as_mut())
    }

    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        (Some(view.entities()), view.as_non_null_ptr())
    }

    unsafe fn get_dense<'a>(
        ptr: Self::DenseParts<'a>,
        index: usize,
        _entity: Entity,
    ) -> Self::Item<'a> {
        ptr.add(index).as_mut()
    }

    unsafe fn slice<'a>(
        ptr: Self::DenseParts<'a>,
        _entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        slice::from_raw_parts_mut(ptr.as_ptr(), range.end - range.start)
    }
}

unsafe impl<T> QueryElem for Option<&'_ T>
where
    T: Component,
{
    type View<'a> = View<'a, T>;
    type Item<'a> = Option<&'a T>;
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type SparseParts<'a> = (&'a SparseVec, NonNull<T>);
    type DenseParts<'a> = (&'a SparseVec, NonNull<T>);

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow::<T>(), None)
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(view.get(entity))
    }

    fn split_sparse<'a>(_view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    fn sparse_contains(_sparse: Self::Sparse<'_>, _entity: Entity) -> bool {
        true
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    unsafe fn get_sparse<'a>(
        (sparse, ptr): Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        Some(
            sparse
                .get_sparse(entity.sparse())
                .map(|entity| ptr.add(entity.dense()).as_ref()),
        )
    }

    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    unsafe fn get_dense<'a>(
        (sparse, ptr): Self::DenseParts<'a>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'a> {
        sparse
            .get_sparse(entity.sparse())
            .map(|entity| ptr.add(entity.dense()).as_ref())
    }

    unsafe fn slice<'a>(
        _parts: Self::DenseParts<'_>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        ()
    }
}

unsafe impl<T> QueryElem for Option<&'_ mut T>
where
    T: Component,
{
    type View<'a> = ViewMut<'a, T>;
    type Item<'a> = Option<&'a mut T>;
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type SparseParts<'a> = (&'a SparseVec, NonNull<T>);
    type DenseParts<'a> = (&'a SparseVec, NonNull<T>);

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow_mut::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow_mut::<T>(), None)
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(view.get_mut(entity))
    }

    fn split_sparse<'a>(_view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    fn sparse_contains(_sparse: Self::Sparse<'_>, _entity: Entity) -> bool {
        true
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    unsafe fn get_sparse<'a>(
        (sparse, ptr): Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        Some(
            sparse
                .get_sparse(entity.sparse())
                .map(|entity| ptr.add(entity.dense()).as_mut()),
        )
    }

    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    unsafe fn get_dense<'a>(
        (sparse, ptr): Self::DenseParts<'a>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'a> {
        sparse
            .get_sparse(entity.sparse())
            .map(|entity| ptr.add(entity.dense()).as_mut())
    }

    unsafe fn slice<'a>(
        _parts: Self::DenseParts<'_>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        ()
    }
}
