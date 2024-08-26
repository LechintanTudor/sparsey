mod iter;
mod query_elem;
mod world_query;

pub use self::iter::*;
pub use self::query_elem::*;
pub use self::world_query::*;

use crate::component::QueryGroupInfo;
use crate::entity::Entity;
use crate::World;
use core::ops::Range;

pub trait Query {
    type View<'a>;
    type Item<'a>;
    type Slice<'a>;
    type Sparse<'a>: Copy;
    type SparseParts<'a>: Copy;
    type DenseParts<'a>: Copy;

    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>);

    #[must_use]
    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>>;

    #[must_use]
    fn split_sparse<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>);

    #[must_use]
    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>);

    #[must_use]
    unsafe fn get_sparse<'a>(
        parts: Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>>;

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

impl Query for () {
    type View<'a> = ();
    type Item<'a> = ();
    type Slice<'a> = ();
    type Sparse<'a> = ();
    type SparseParts<'a> = ();
    type DenseParts<'a> = ();

    #[inline]
    fn borrow(_world: &World) -> Self::View<'_> {
        ()
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
    fn split_sparse<'a>(_view: &'a Self::View<'a>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        (None, ())
    }

    #[inline]
    fn sparse_contains_all(_sparse: Self::Sparse<'_>, _entity: Entity) -> bool {
        true
    }

    #[inline]
    fn sparse_contains_none(_sparse: Self::Sparse<'_>, _entity: Entity) -> bool {
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
        _parts: Self::SparseParts<'a>,
        _entity: Entity,
    ) -> Option<Self::Item<'a>> {
        Some(())
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
        _entity: Entity,
    ) -> Self::Item<'a> {
        ()
    }

    #[inline]
    unsafe fn slice<'a>(
        _parts: Self::DenseParts<'a>,
        _entities: &'a [Entity],
        _range: Range<usize>,
    ) -> Self::Slice<'a> {
        ()
    }
}

impl<Q> Query for Q
where
    Q: QueryElem,
{
    type View<'a> = <Q as QueryElem>::View<'a>;
    type Item<'a> = <Q as QueryElem>::Item<'a>;
    type Slice<'a> = <Q as QueryElem>::Slice<'a>;
    type Sparse<'a> = <Q as QueryElem>::Sparse<'a>;
    type SparseParts<'a> = <Q as QueryElem>::SparseParts<'a>;
    type DenseParts<'a> = <Q as QueryElem>::DenseParts<'a>;

    fn borrow(world: &World) -> Self::View<'_> {
        <Q as QueryElem>::borrow(world)
    }

    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
        let (view, info) = <Q as QueryElem>::borrow_with_group_info(world);
        let info = info.map_or(QueryGroupInfo::Empty, QueryGroupInfo::One);
        (view, Some(info))
    }

    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool {
        <Q as QueryElem>::contains(view, entity)
    }

    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool {
        !<Q as QueryElem>::contains(view, entity)
    }

    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        <Q as QueryElem>::get(view, entity)
    }

    fn split_sparse<'a>(view: &'a Self::View<'_>) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
        <Q as QueryElem>::split_sparse(view)
    }

    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        <Q as QueryElem>::sparse_contains(sparse, entity)
    }

    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        !<Q as QueryElem>::sparse_contains(sparse, entity)
    }

    fn split_sparse_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
        <Q as QueryElem>::split_sparse_parts(view)
    }

    unsafe fn get_sparse<'a>(
        parts: Self::SparseParts<'a>,
        entity: Entity,
    ) -> Option<Self::Item<'a>> {
        <Q as QueryElem>::get_sparse(parts, entity)
    }

    fn split_dense_parts<'a>(
        view: &'a Self::View<'_>,
    ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
        <Q as QueryElem>::split_dense_parts(view)
    }

    unsafe fn get_dense<'a>(
        parts: Self::DenseParts<'a>,
        index: usize,
        entity: Entity,
    ) -> Self::Item<'a> {
        <Q as QueryElem>::get_dense(parts, index, entity)
    }

    unsafe fn slice<'a>(
        parts: Self::DenseParts<'a>,
        entities: &'a [Entity],
        range: Range<usize>,
    ) -> Self::Slice<'a> {
        <Q as QueryElem>::slice(parts, entities, range)
    }
}

macro_rules! impl_query {
    ($(($Ty:ident, $idx:tt)),+) => {
        impl<$($Ty),+> Query for ($($Ty,)+)
        where
            $($Ty: QueryElem,)+
        {
            type View<'a> = ($($Ty::View<'a>,)+);
            type Item<'a> = ($($Ty::Item<'a>,)+);
            type Slice<'a> = ($($Ty::Slice<'a>,)+);
            type Sparse<'a> = ($($Ty::Sparse<'a>,)+);
            type SparseParts<'a> = ($($Ty::SparseParts<'a>,)+);
            type DenseParts<'a> = ($($Ty::DenseParts<'a>,)+);

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
                Some(($($Ty::get(&mut view.$idx, entity)?,)+))
            }

            fn split_sparse<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::Sparse<'a>) {
                let mut entities = Option::<&[Entity]>::None;

                let sparse = ($({
                    let (view_entities, sparse) = $Ty::split_sparse(&view.$idx);

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

            fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
                $($Ty::sparse_contains(sparse.$idx, entity))&&+
            }

            fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
                $(!$Ty::sparse_contains(sparse.$idx, entity))&&+
            }

            fn split_sparse_parts<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::SparseParts<'a>) {
                let mut entities = Option::<&[Entity]>::None;

                let part = ($({
                    let (view_entities, part) = $Ty::split_sparse_parts(&view.$idx);

                    if let Some(view_entities) = view_entities {
                        if let Some(old_entities) = entities {
                            if view_entities.len() < old_entities.len() {
                                entities = Some(view_entities);
                            }
                        } else {
                            entities = Some(view_entities);
                        }
                    }

                    part
                },)+);

                (entities, part)
            }

            unsafe fn get_sparse<'a>(
                parts: Self::SparseParts<'a>,
                entity: Entity,
            ) -> Option<Self::Item<'a>> {
                Some(($($Ty::get_sparse(parts.$idx, entity)?,)+))
            }

            fn split_dense_parts<'a>(
                view: &'a Self::View<'_>,
            ) -> (Option<&'a [Entity]>, Self::DenseParts<'a>) {
                let mut entities = Option::<&[Entity]>::None;

                let part = ($({
                    let (view_entities, part) = $Ty::split_dense_parts(&view.$idx);

                    if entities.is_none() && view_entities.is_some() {
                        entities = view_entities;
                    }

                    part
                },)+);

                (entities, part)
            }

            unsafe fn get_dense<'a>(
                parts: Self::DenseParts<'a>,
                index: usize,
                entity: Entity,
            ) -> Self::Item<'a> {
                ($($Ty::get_dense(parts.$idx, index, entity),)+)
            }

            unsafe fn slice<'a>(
                parts: Self::DenseParts<'a>,
                entities: &'a [Entity],
                range: Range<usize>,
            ) -> Self::Slice<'a> {
               ($($Ty::slice(parts.$idx, entities, range.clone()),)+)
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
