//! Query and iterate entities and components.

mod iter;
mod query_all;
mod query_one;
mod query_part;

#[cfg(feature = "parallel")]
mod par_iter;

pub use self::iter::*;
pub use self::query_all::*;
pub use self::query_one::*;
pub use self::query_part::*;

#[cfg(feature = "parallel")]
pub use self::par_iter::*;

use crate::component::QueryGroupInfo;
use crate::entity::Entity;
use crate::World;
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

/// Trait for querying and iterating one or more views.
///
/// # Safety
///
/// This trait is considered an implementation detail and cannot be safely
/// implemented outside the crate.
pub unsafe trait Query {
    /// View borrowed from a [`World`].
    type View<'a>;

    /// Item type returned by queries.
    type Item<'a>: Send;

    /// Type returned by [`slice`](Self::slice_raw) operations.
    type Slice<'a>;

    /// [`SparseVec`](crate::entity::SparseVec) type used for sparse iteration.
    type Sparse<'a>: Copy;

    /// Data used for sparse and dense iteration.
    type Data<'a>: Copy;

    /// Borrows a view from the `world`.
    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    /// Borrows a view from the `world` along with grouping information.
    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>);

    /// Returns whether `entity` is present in all parts of the `view`.
    #[must_use]
    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool;

    /// Returns whether `entity` is present in none of the parts of the `view`.
    #[must_use]
    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool;

    /// Returns the item mapped to `entity`, if any.
    #[must_use]
    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>>;

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

    /// Returns whether the sparse index is present in all sparse vecs.
    #[must_use]
    fn contains_all_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool;

    /// Returns whether the sparse index is present in none of the sparse vecs.
    #[must_use]
    fn contains_none_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool;

    /// Returns the item mapped to `entity`, if any.
    #[must_use]
    unsafe fn get_sparse_raw<'a>(
        sparse: Self::Sparse<'a>,
        data: Self::Data<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>>;

    /// Returns the item mapped to `index` or `entity`.
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

#[allow(clippy::unused_unit)]
unsafe impl Query for () {
    type View<'a> = ();
    type Item<'a> = ();
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type Data<'a> = ();

    #[inline]
    fn borrow(_world: &World) -> Self::View<'_> {
        // Empty
    }

    #[inline]
    fn borrow_with_group_info(_world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
        ((), Some(QueryGroupInfo::Empty))
    }

    #[inline]
    fn contains_all(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    fn contains_none(_view: &Self::View<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    fn get<'a>(_view: &'a mut Self::View<'_>, _entity: Entity) -> Option<Self::Item<'a>> {
        Some(())
    }

    #[inline]
    fn split_filter_parts<'a>(
        _view: &'a Self::View<'a>,
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
    fn contains_all_raw(_sparse: Self::Sparse<'_>, _sparse_index: usize) -> bool {
        true
    }

    #[inline]
    fn contains_none_raw(_sparse: Self::Sparse<'_>, _sparse_index: usize) -> bool {
        true
    }

    #[inline]
    unsafe fn get_sparse_raw<'a>(
        _sparse: Self::Sparse<'a>,
        _data: Self::Data<'a>,
        _entity: Entity,
    ) -> Option<Self::Item<'a>> {
        Some(())
    }

    #[inline]
    unsafe fn get_dense_raw(
        _data: Self::Data<'_>,
        _index: usize,
        _entity: Entity,
    ) -> Self::Item<'_> {
        // Empty
    }

    #[inline]
    unsafe fn slice_raw<'a>(
        _data: Self::Data<'a>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        // Empty
    }
}

unsafe impl<Q> Query for Q
where
    Q: QueryPart,
{
    type View<'a> = <Q as QueryPart>::View<'a>;
    type Item<'a> = <Q as QueryPart>::Item<'a>;
    type Slice<'a> = <Q as QueryPart>::Slice<'a>;
    type Sparse<'a> = <Q as QueryPart>::Sparse<'a>;
    type Data<'a> = <Q as QueryPart>::Data<'a>;

    fn borrow(world: &World) -> Self::View<'_> {
        <Q as QueryPart>::borrow(world)
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
        let (view, info) = <Q as QueryPart>::borrow_with_group_info(world);
        let info = info.map_or(QueryGroupInfo::Empty, QueryGroupInfo::One);
        (view, Some(info))
    }

    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool {
        <Q as QueryPart>::contains(view, entity)
    }

    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool {
        !<Q as QueryPart>::contains(view, entity)
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        let key = <Q as QueryPart>::get_sparse_key(view, entity)?;
        unsafe { Some(<Q as QueryPart>::get_sparse(view, key)) }
    }

    fn split_filter_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        <Q as QueryPart>::split_filter_parts(view)
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
        <Q as QueryPart>::split_sparse_parts(view)
    }

    fn split_dense_parts<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Data<'a>) {
        <Q as QueryPart>::split_dense_parts(view)
    }

    fn contains_all_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
        <Q as QueryPart>::contains_raw(sparse, sparse_index)
    }

    fn contains_none_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
        !<Q as QueryPart>::contains_raw(sparse, sparse_index)
    }

    unsafe fn get_sparse_raw<'a>(
        sparse: Self::Sparse<'_>,
        data: Self::Data<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        let key = <Q as QueryPart>::get_sparse_key_raw(sparse, entity)?;
        Some(<Q as QueryPart>::get_sparse_raw(data, key))
    }

    unsafe fn get_dense_raw(data: Self::Data<'_>, index: usize, entity: Entity) -> Self::Item<'_> {
        <Q as QueryPart>::get_dense_raw(data, index, entity)
    }

    unsafe fn slice_raw<'a>(
        data: Self::Data<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        <Q as QueryPart>::slice_raw(data, entities, range)
    }
}

macro_rules! impl_query {
    ($(($Ty:ident, $idx:tt)),+) => {
        unsafe impl<$($Ty),+> Query for ($($Ty,)+)
        where
            $($Ty: QueryPart,)+
        {
            type View<'a> = ($($Ty::View<'a>,)+);
            type Item<'a> = ($($Ty::Item<'a>,)+);
            type Slice<'a> = ($($Ty::Slice<'a>,)+);
            type Sparse<'a> = ($($Ty::Sparse<'a>,)+);
            type Data<'a> = ($($Ty::Data<'a>,)+);

            fn borrow(world: &World) -> Self::View<'_> {
                ($($Ty::borrow(world),)+)
            }

            fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
                let view_and_group_info = ($($Ty::borrow_with_group_info(world),)+);

                let get_group_info = || -> Option<QueryGroupInfo> {
                    let mut group_info = QueryGroupInfo::Empty;

                    $(
                        if let Some(info) = &view_and_group_info.$idx.1 {
                            group_info = group_info.add_view(info)?;
                        }
                    )+

                    Some(group_info)
                };

                (
                    ($(view_and_group_info.$idx.0,)+),
                    get_group_info(),
                )
            }

            fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool {
                $($Ty::contains(&view.$idx, entity))&&+
            }

            fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool {
                $(!$Ty::contains(&view.$idx, entity))&&+
            }

            fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
                let key = ($($Ty::get_sparse_key(&view.$idx, entity)?,)+);
                unsafe { Some(($($Ty::get_sparse(&mut view.$idx, key.$idx),)+)) }
            }

            fn split_filter_parts<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
                let mut entities = Option::<&[Entity]>::None;

                let sparse = ($({
                    let (view_entities, sparse) = $Ty::split_filter_parts(&view.$idx);

                    if let Some(view_entities) = view_entities {
                        if let Some(old_entities) = entities {
                            if view_entities.len() < old_entities.len() {
                                entities = Some(view_entities);
                            }
                        } else {
                            entities = Some(view_entities);
                        }
                    }

                    sparse
                },)+);

                (entities, sparse)
            }

            fn split_sparse_parts<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Data<'a>) {
                let mut entities = Option::<&[Entity]>::None;
                let mut sparse: MaybeUninit<Self::Sparse<'a>> = MaybeUninit::uninit();
                let mut data: MaybeUninit<Self::Data<'a>> = MaybeUninit::uninit();

                $({
                    let (view_entities, view_sparse, view_data)
                        = $Ty::split_sparse_parts(&view.$idx);

                    if let Some(view_entities) = view_entities {
                        if let Some(old_entities) = entities {
                            if view_entities.len() < old_entities.len() {
                                entities = Some(view_entities);
                            }
                        } else {
                            entities = Some(view_entities);
                        }
                    }

                    unsafe {
                        ptr::addr_of_mut!((*sparse.as_mut_ptr()).$idx).write(view_sparse);
                        ptr::addr_of_mut!((*data.as_mut_ptr()).$idx).write(view_data);
                    }
                })+;

                unsafe {
                    (entities, sparse.assume_init(), data.assume_init())
                }
            }

            fn split_dense_parts<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::Data<'a>) {
                let mut entities = Option::<&[Entity]>::None;

                let data = ($({
                    let (view_entities, data) = $Ty::split_dense_parts(&view.$idx);

                    if entities.is_none() && view_entities.is_some() {
                        entities = view_entities;
                    }

                    data
                },)+);

                (entities, data)
            }

            fn contains_all_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
                $($Ty::contains_raw(sparse.$idx, sparse_index))&&+
            }

            fn contains_none_raw(sparse: Self::Sparse<'_>, sparse_index: usize) -> bool {
                $(!$Ty::contains_raw(sparse.$idx, sparse_index))&&+
            }

            unsafe fn get_sparse_raw<'a>(
                sparse: Self::Sparse<'a>,
                data: Self::Data<'a>,
                entity: Entity,
            ) -> Option<Self::Item<'a>> {
                let key = ($($Ty::get_sparse_key_raw(sparse.$idx, entity)?,)+);
                Some(($($Ty::get_sparse_raw(data.$idx, key.$idx),)+))
            }

            unsafe fn get_dense_raw(
                data: Self::Data<'_>,
                index: usize,
                entity: Entity,
            ) -> Self::Item<'_> {
                ($($Ty::get_dense_raw(data.$idx, index, entity),)+)
            }

            unsafe fn slice_raw<'a>(
                data: Self::Data<'a>,
                entities: &'a [Entity],
                range: Range<usize>,
            ) -> Self::Slice<'a> {
               ($($Ty::slice_raw(data.$idx, entities, range.clone()),)+)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query!((A, 0));
    impl_query!((A, 0), (B, 1));
    impl_query!((A, 0), (B, 1), (C, 2));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
