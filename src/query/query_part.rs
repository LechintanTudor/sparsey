use crate::component::{Component, View, ViewGroupInfo, ViewMut};
use crate::entity::{Entity, SparseVec};
use crate::World;
use core::ops::Range;
use core::ptr::NonNull;
use core::slice;

/// Trait for querying and iterating a view.
///
/// # Safety
///
/// This trait is considered an implementation detail and cannot be safely
/// implemented outside the crate.
pub unsafe trait QueryPart {
    /// View borrowed from a [`World`].
    type View<'a>;

    /// Item type returned by queries.
    type Item<'a>: Send;

    /// Slice returned by [`slice`](Self::slice_raw) operations.
    type Slice<'a>;

    /// [`SparseVec`](crate::entity::SparseVec) type used for sparse iteration.
    type Sparse<'a>: Copy;

    /// Key used to access dense data.
    type SparseKey;

    /// Data used for sparse and dense iteration.
    type Data<'a>: Copy;

    /// Borrows a view from the `world`.
    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    /// Borrows a view from the `world` along with grouping information.
    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>);

    /// Returns whether `entity` is present in the view.
    #[must_use]
    fn contains(view: &Self::View<'_>, entity: Entity) -> bool;

    /// Returns the sparse key mapped to `entity`, if any.
    #[must_use]
    fn get_sparse_key(view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey>;

    /// Returns the item mapped to `key`, if any.
    #[must_use]
    unsafe fn get_sparse<'a>(view: &'a mut Self::View<'_>, key: Self::SparseKey) -> Self::Item<'a>;

    /// Splits the view into its entities and sparse vecs.
    #[must_use]
    fn split_filter_parts<'a>(view: &'a Self::View<'_>)
        -> (Option<&'a [Entity]>, Self::Sparse<'a>);

    /// Splits the view into its entities, sparse vecs and data.
    #[must_use]
    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>);

    /// Splits the view into its entities and data.
    #[must_use]
    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>);

    /// Returns whether the sparse index is present in the sparse vecs.
    #[must_use]
    fn contains_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool;

    /// Returns the sparse key extracted from the sparse vecs.
    #[must_use]
    fn get_sparse_key_raw(sparse: Self::Sparse<'_>, entity: Entity) -> Option<Self::SparseKey>;

    /// Returns the sparse data that can be accessed with the dense key.
    #[must_use]
    unsafe fn get_sparse_raw(data: Self::Data<'_>, key: Self::SparseKey) -> Self::Item<'_>;

    /// Returns the item at the given `index` or `entity`.
    #[must_use]
    unsafe fn get_dense_raw(data: Self::Data<'_>, index: usize, entity: Entity) -> Self::Item<'_>;

    /// Slices the data at the given `range`.
    #[must_use]
    unsafe fn slice_raw<'a>(
        data: Self::Data<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a>;
}

unsafe impl QueryPart for Entity {
    type View<'a> = ();
    type Item<'a> = Entity;
    type Slice<'a> = &'a [Entity];
    type Sparse<'a> = ();
    type SparseKey = Entity;
    type Data<'a> = ();

    #[inline]
    fn borrow(_world: &World) -> Self::View<'_> {
        // Empty
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
    fn get_sparse_key(_view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(entity)
    }

    #[inline]
    unsafe fn get_sparse<'a>(
        _view: &'a mut Self::View<'_>,
        key: Self::SparseKey,
    ) -> Self::Item<'a> {
        key
    }

    #[inline]
    fn split_filter_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    #[inline]
    fn split_sparse_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        (None, (), ())
    }

    #[inline]
    fn split_dense_parts<'a>(_view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        (None, ())
    }

    #[inline]
    fn contains_raw(_sparse: Self::Sparse<'_>, _sparse_index: usize) -> bool {
        true
    }

    #[inline]
    fn get_sparse_key_raw<'a>(
        _sparse: Self::Sparse<'_>,
        entity: Entity,
    ) -> Option<Self::SparseKey> {
        Some(entity)
    }

    #[inline]
    unsafe fn get_sparse_raw(_data: Self::Data<'_>, key: Self::SparseKey) -> Self::Item<'_> {
        key
    }

    #[inline]
    unsafe fn get_dense_raw(
        _part: Self::Data<'_>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'_> {
        entity
    }

    #[inline]
    unsafe fn slice_raw<'a>(
        _data: Self::Data<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        entities.get_unchecked(range)
    }
}

unsafe impl<T> QueryPart for &'_ T
where
    T: Component,
{
    type View<'a> = View<'a, T>;
    type Item<'a> = &'a T;
    type Slice<'a> = &'a [T];
    type Sparse<'a> = &'a SparseVec;
    type SparseKey = usize;
    type Data<'a> = NonNull<T>;

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

    fn get_sparse_key(view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey> {
        view.sparse().get(entity).map(|i| i as usize)
    }

    unsafe fn get_sparse<'a>(view: &'a mut Self::View<'_>, key: Self::SparseKey) -> Self::Item<'a> {
        view.as_non_null_ptr().add(key).as_ref()
    }

    fn split_filter_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (Some(view.entities()), view.sparse())
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        (Some(view.entities()), view.sparse(), view.as_non_null_ptr())
    }

    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        (Some(view.entities()), view.as_non_null_ptr())
    }

    fn contains_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
        sparse.contains_sparse(sparse_index)
    }

    fn get_sparse_key_raw<'a>(sparse: Self::Sparse<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(sparse.get_sparse(entity.sparse())? as usize)
    }

    unsafe fn get_sparse_raw(data: Self::Data<'_>, key: Self::SparseKey) -> Self::Item<'_> {
        data.add(key).as_ref()
    }

    unsafe fn get_dense_raw(ptr: Self::Data<'_>, index: usize, _entity: Entity) -> Self::Item<'_> {
        ptr.add(index).as_ref()
    }

    unsafe fn slice_raw<'a>(
        data: Self::Data<'a>,
        _entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        slice::from_raw_parts(data.as_ptr(), range.end - range.start)
    }
}

unsafe impl<T> QueryPart for &'_ mut T
where
    T: Component,
{
    type View<'a> = ViewMut<'a, T>;
    type Item<'a> = &'a mut T;
    type Slice<'a> = &'a mut [T];
    type Sparse<'a> = &'a SparseVec;
    type SparseKey = usize;
    type Data<'a> = NonNull<T>;

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

    fn get_sparse_key(view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey> {
        view.sparse().get(entity).map(|i| i as usize)
    }

    unsafe fn get_sparse<'a>(view: &'a mut Self::View<'_>, key: Self::SparseKey) -> Self::Item<'a> {
        view.as_non_null_ptr().add(key).as_mut()
    }

    fn split_filter_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (Some(view.entities()), view.sparse())
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        (Some(view.entities()), view.sparse(), view.as_non_null_ptr())
    }

    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        (Some(view.entities()), view.as_non_null_ptr())
    }

    fn contains_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
        sparse.contains_sparse(sparse_index)
    }

    fn get_sparse_key_raw(sparse: Self::Sparse<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(sparse.get_sparse(entity.sparse())? as usize)
    }

    unsafe fn get_sparse_raw(data: Self::Data<'_>, key: Self::SparseKey) -> Self::Item<'_> {
        data.add(key).as_mut()
    }

    unsafe fn get_dense_raw(ptr: Self::Data<'_>, index: usize, _entity: Entity) -> Self::Item<'_> {
        ptr.add(index).as_mut()
    }

    unsafe fn slice_raw<'a>(
        data: Self::Data<'a>,
        _entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        slice::from_raw_parts_mut(data.as_ptr(), range.end - range.start)
    }
}

unsafe impl<T> QueryPart for Option<&'_ T>
where
    T: Component,
{
    type View<'a> = View<'a, T>;
    type Item<'a> = Option<&'a T>;
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type SparseKey = Entity;
    type Data<'a> = (&'a SparseVec, NonNull<T>);

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow::<T>(), None)
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    fn get_sparse_key(_view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(entity)
    }

    unsafe fn get_sparse<'a>(view: &'a mut Self::View<'_>, key: Self::SparseKey) -> Self::Item<'a> {
        view.get(key)
    }

    fn split_filter_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        (None, (), (view.sparse(), view.as_non_null_ptr()))
    }

    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    fn contains_raw(_sparse: Self::Sparse<'_>, _sparse_index: usize) -> bool {
        true
    }

    fn get_sparse_key_raw(_sparse: Self::Sparse<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(entity)
    }

    unsafe fn get_sparse_raw(
        (sparse, ptr): Self::Data<'_>,
        entity: Self::SparseKey,
    ) -> Self::Item<'_> {
        sparse
            .get_sparse(entity.sparse())
            .map(|dense| ptr.add(dense as usize).as_ref())
    }

    unsafe fn get_dense_raw(
        (sparse, ptr): Self::Data<'_>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'_> {
        sparse
            .get_sparse(entity.sparse())
            .map(|dense| ptr.add(dense as usize).as_ref())
    }

    unsafe fn slice_raw<'a>(
        _data: Self::Data<'_>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        // Empty
    }
}

unsafe impl<T> QueryPart for Option<&'_ mut T>
where
    T: Component,
{
    type View<'a> = ViewMut<'a, T>;
    type Item<'a> = Option<&'a mut T>;
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type SparseKey = Entity;
    type Data<'a> = (&'a SparseVec, NonNull<T>);

    fn borrow(world: &World) -> Self::View<'_> {
        world.borrow_mut::<T>()
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<ViewGroupInfo>) {
        (world.borrow_mut::<T>(), None)
    }

    fn contains(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    fn get_sparse_key(_view: &Self::View<'_>, entity: Entity) -> Option<Self::SparseKey> {
        Some(entity)
    }

    unsafe fn get_sparse<'a>(view: &'a mut Self::View<'_>, key: Self::SparseKey) -> Self::Item<'a> {
        view.get_mut(key)
    }

    fn split_filter_parts<'a>(
        _view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        (None, (), (view.sparse(), view.as_non_null_ptr()))
    }

    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        (None, (view.sparse(), view.as_non_null_ptr()))
    }

    fn contains_raw(_sparse: Self::Sparse<'_>, _sparse_index: usize) -> bool {
        true
    }

    fn get_sparse_key_raw<'a>(
        _sparse: Self::Sparse<'_>,
        entity: Entity,
    ) -> Option<Self::SparseKey> {
        Some(entity)
    }

    unsafe fn get_sparse_raw(
        (sparse, ptr): Self::Data<'_>,
        entity: Self::SparseKey,
    ) -> Self::Item<'_> {
        sparse
            .get_sparse(entity.sparse())
            .map(|dense| ptr.add(dense as usize).as_mut())
    }

    unsafe fn get_dense_raw(
        (sparse, ptr): Self::Data<'_>,
        _index: usize,
        entity: Entity,
    ) -> Self::Item<'_> {
        sparse
            .get_sparse(entity.sparse())
            .map(|dense| ptr.add(dense as usize).as_mut())
    }

    unsafe fn slice_raw<'a>(
        _data: Self::Data<'_>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        // Empty
    }
}
