use crate::query::{ComponentView, QueryGroupInfo};
use crate::storage::{Entity, SparseArray};
use std::ops::RangeBounds;

/// Allows getting, including or excluding components in a query.
pub trait QueryPart {
    /// References to components returned by the query.
    type Refs<'a>
    where
        Self: 'a;

    /// References to sparse arrays obtained from splitting the query part.
    type Sparse<'a>: Copy
    where
        Self: 'a;

    /// Pointers to components returned by the query.
    type Ptrs: Copy;

    /// Slices of components returned by the query.
    type Slices<'a>
    where
        Self: 'a;

    /// Returns the components mapped to `entity` if the exist.
    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a;

    /// Returns whether `entity` contains all the components specified by the query.
    fn contains_all(self, entity: Entity) -> bool;

    /// Returns whether `entity` contains none of the components specified by the query.
    fn contains_none(self, entity: Entity) -> bool;

    /// Returns the group info of the query.
    fn group_info<'a>(&'a self) -> Option<QueryGroupInfo<'a>>
    where
        Self: 'a;

    /// Splits the query part for sparse iteration.
    fn split_for_sparse_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a;

    /// Splits the query part for dense iteration.
    fn split_for_dense_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Ptrs)
    where
        Self: 'a;

    /// Splits the query part for filtering iterators.
    fn split_for_filtering<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>)
    where
        Self: 'a;

    /// Returns a slice of entitites that belongs to one of the [`ComponentViews`](ComponentView)
    /// specified by the query.
    fn into_any_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a;

    /// Applies an offset to the given pointers.
    unsafe fn offset_ptrs(ptrs: Self::Ptrs, offset: usize) -> Self::Ptrs;

    /// Returns the components at the given sparse index, if they exist.
    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a;

    /// Returns the components at the given dense index, if they exist.
    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a;

    /// Returns whether all of the sparse arrays contain `entity`.
    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a;

    /// Returns whether none of the sparse arrays contain `entity`.
    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a;

    /// Returns a slice containing all entities in the given `range`.
    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>;

    /// Returns slices containing all components in the given `range`.
    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slices<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>;

    /// Returns all entities and components in the given `range` as slices.
    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slices<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>;
}

impl QueryPart for () {
    type Refs<'a> = ();
    type Sparse<'a> = ();
    type Ptrs = ();
    type Slices<'a> = ();

    fn get<'a>(self, _entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        None
    }

    fn contains_all(self, _entity: Entity) -> bool {
        true
    }

    fn contains_none(self, _entity: Entity) -> bool {
        true
    }

    fn group_info<'a>(&'a self) -> Option<QueryGroupInfo<'a>>
    where
        Self: 'a,
    {
        Some(QueryGroupInfo::Empty)
    }

    fn split_for_sparse_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a,
    {
        (None, (), ())
    }

    fn split_for_dense_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Ptrs)
    where
        Self: 'a,
    {
        (None, ())
    }

    fn split_for_filtering<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>)
    where
        Self: 'a,
    {
        (None, ())
    }

    fn into_any_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a,
    {
        None
    }

    unsafe fn offset_ptrs(_ptrs: Self::Ptrs, _offset: usize) -> Self::Ptrs {
        ()
    }

    unsafe fn sparse_get<'a>(
        _sparse: Self::Sparse<'a>,
        _ptrs: Self::Ptrs,
        _sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        None
    }

    unsafe fn dense_get<'a>(_ptrs: Self::Ptrs, _dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        ()
    }

    fn sparse_contains_all<'a>(_sparse: Self::Sparse<'a>, _entity: Entity) -> bool
    where
        Self: 'a,
    {
        true
    }

    fn sparse_contains_none<'a>(_sparse: Self::Sparse<'a>, _entity: Entity) -> bool
    where
        Self: 'a,
    {
        true
    }

    unsafe fn get_entities_unchecked<'a, R>(self, _range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        &[]
    }

    unsafe fn get_components_unchecked<'a, R>(self, _range: R) -> Self::Slices<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        ()
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        _range: R,
    ) -> (&'a [Entity], Self::Slices<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        (&[], ())
    }
}

impl<C> QueryPart for C
where
    C: ComponentView,
{
    type Refs<'a> = C::Ref<'a> where Self: 'a;
    type Sparse<'a> = &'a SparseArray where Self: 'a;
    type Ptrs = C::Ptr;
    type Slices<'a> = C::Slice<'a> where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        ComponentView::get(self, entity)
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self, entity)
    }

    fn group_info<'a>(&'a self) -> Option<QueryGroupInfo<'a>>
    where
        Self: 'a,
    {
        Some(QueryGroupInfo::Single {
            len: ComponentView::len(self),
            info: ComponentView::group_info(self),
        })
    }

    fn split_for_sparse_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, sparse, components) = ComponentView::split_for_iteration(self);
        (Some(entities), sparse, components)
    }

    fn split_for_dense_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, _, components) = ComponentView::split_for_iteration(self);
        (Some(entities), components)
    }

    fn split_for_filtering<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>)
    where
        Self: 'a,
    {
        let (entities, sparse, _) = ComponentView::split_for_iteration(self);
        (Some(entities), sparse)
    }

    fn into_any_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a,
    {
        let (entities, _, _) = ComponentView::split_for_iteration(self);
        Some(entities)
    }

    unsafe fn offset_ptrs(ptrs: Self::Ptrs, offset: usize) -> Self::Ptrs {
        <C as ComponentView>::offset_ptr(ptrs, offset)
    }

    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        let dense_index = sparse.get_from_sparse(sparse_index)?;
        Some(<C as ComponentView>::get_from_ptr(ptrs, dense_index))
    }

    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        <C as ComponentView>::get_from_ptr(ptrs, dense_index)
    }

    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        sparse.contains(entity)
    }

    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        !sparse.contains(entity)
    }

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        ComponentView::get_entities_unchecked(self, range)
    }

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slices<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        ComponentView::get_components_unchecked(self, range)
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slices<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        ComponentView::get_entities_and_components_unchecked(self, range)
    }
}

impl<C> QueryPart for (C,)
where
    C: ComponentView,
{
    type Refs<'a> = (C::Ref<'a>,) where Self: 'a;
    type Sparse<'a> = (&'a SparseArray,) where Self: 'a;
    type Ptrs = (C::Ptr,);
    type Slices<'a> = (C::Slice<'a>,) where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        Some((ComponentView::get(self.0, entity)?,))
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self.0, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self.0, entity)
    }

    fn group_info<'a>(&'a self) -> Option<QueryGroupInfo<'a>>
    where
        Self: 'a,
    {
        Some(QueryGroupInfo::Single {
            len: ComponentView::len(&self.0),
            info: ComponentView::group_info(&self.0),
        })
    }

    fn split_for_sparse_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, sparse, components) = ComponentView::split_for_iteration(self.0);
        (Some(entities), (sparse,), (components,))
    }

    fn split_for_dense_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, _, components) = ComponentView::split_for_iteration(self.0);
        (Some(entities), (components,))
    }

    fn split_for_filtering<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>)
    where
        Self: 'a,
    {
        let (entities, sparse, _) = ComponentView::split_for_iteration(self.0);
        (Some(entities), (sparse,))
    }

    fn into_any_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a,
    {
        let (entities, _, _) = ComponentView::split_for_iteration(self.0);
        Some(entities)
    }

    unsafe fn offset_ptrs(ptrs: Self::Ptrs, offset: usize) -> Self::Ptrs {
        (<C as ComponentView>::offset_ptr(ptrs.0, offset),)
    }

    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        let dense_index = sparse.0.get_from_sparse(sparse_index)?;
        Some((<C as ComponentView>::get_from_ptr(ptrs.0, dense_index),))
    }

    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        (<C as ComponentView>::get_from_ptr(ptrs.0, dense_index),)
    }

    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        sparse.0.contains(entity)
    }

    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        !sparse.0.contains(entity)
    }

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        ComponentView::get_entities_unchecked(self.0, range)
    }

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slices<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        (ComponentView::get_components_unchecked(self.0, range),)
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slices<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let (entities, components) =
            ComponentView::get_entities_and_components_unchecked(self.0, range);

        (entities, (components,))
    }
}

macro_rules! replace_with_sparse_array_ref {
    ($old:tt) => {
        &'a SparseArray
    };
}

macro_rules! query_group_info {
    ($first:expr, $($other:expr),+) => {
        Some(QueryGroupInfo::Multiple($first? $(.combine($other?)?)+))
    };
}

macro_rules! split_for_sparse_iteration {
    (($first:expr, $_:tt), $(($other:expr, $other_idx:tt)),+) => {
        {
            let (mut entities, first_sparse, first_comp)
                = ComponentView::split_for_iteration($first);

            #[allow(clippy::mixed_read_write_in_expression)]
            let splits = (
                (first_sparse, first_comp),
                $(
                    {
                        let (other_entities, other_sparse, other_comp)
                            = ComponentView::split_for_iteration($other);

                        if other_entities.len() < entities.len() {
                            entities = other_entities;
                        }

                        (other_sparse, other_comp)
                    },
                )+
            );

            let sparse = (first_sparse, $(splits.$other_idx.0),+);
            let comp = (first_comp, $(splits.$other_idx.1),+);

            (Some(entities), sparse, comp)
        }
    };
}

macro_rules! split_for_dense_iteration {
    ($first:expr, $($other:expr),+) => {
        {
            let (entities, _, first_comp) = ComponentView::split_for_iteration($first);

            let comps = (
                first_comp,
                $(ComponentView::split_for_iteration($other).2,)+
            );

            (Some(entities), comps)
        }
    };
}

macro_rules! split_for_filtering {
    ($first:expr, $($other:expr),+) => {
        {
            let (entities, first_sparse, _) = ComponentView::split_for_iteration($first);

            let sparse = (
                first_sparse,
                $(ComponentView::split_for_iteration($other).1,)+
            );

            (Some(entities), sparse)
        }
    };
}

macro_rules! get_entities_and_components_unchecked {
    ($range:expr; $first:expr, $($other:expr),+) => {
        {
            let bounds = ($range.start_bound().cloned(), $range.end_bound().cloned());
            let (entities, first_comp) = $first.get_entities_and_components_unchecked(bounds);
            (entities, (first_comp, $($other.get_components_unchecked(bounds),)+))
        }
    };
}

macro_rules! impl_query_part {
    ($(($comp:ident, $idx:tt)),+) => {
        impl<$($comp),+> QueryPart for ($($comp,)+)
        where
            $($comp: ComponentView,)+
        {
            type Refs<'a> = ($($comp::Ref<'a>,)+) where Self: 'a;
            type Sparse<'a> = ($(replace_with_sparse_array_ref!($comp),)+) where Self: 'a;
            type Ptrs = ($($comp::Ptr,)+);
            type Slices<'a> = ($($comp::Slice<'a>,)+) where Self: 'a;

            fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
            where
                Self: 'a,
            {
                Some((
                    $($comp::get(self.$idx, entity)?,)+
                ))
            }

            fn contains_all(self, entity: Entity) -> bool {
                $($comp::contains(self.$idx, entity))&&+
            }

            fn contains_none(self, entity: Entity) -> bool {
                $(!$comp::contains(self.$idx, entity))&&+
            }

            fn group_info<'a>(&'a self) -> Option<QueryGroupInfo<'a>>
            where
                Self: 'a,
            {
                query_group_info!($(self.$idx.group_info()),+)
            }

            fn split_for_sparse_iteration<'a>(self)
                -> (Option<&'a [Entity]>, Self::Sparse<'a>, Self::Ptrs)
            where
                Self: 'a,
            {
                split_for_sparse_iteration!($((self.$idx, $idx)),+)
            }

            fn split_for_dense_iteration<'a>(self) -> (Option<&'a [Entity]>, Self::Ptrs)
            where
                Self: 'a,
            {
                split_for_dense_iteration!($(self.$idx),+)
            }

            fn split_for_filtering<'a>(self) -> (Option<&'a [Entity]>, Self::Sparse<'a>)
            where
                Self: 'a,
            {
                split_for_filtering!($(self.$idx),+)
            }

            fn into_any_entities<'a>(self) -> Option<&'a [Entity]>
            where
                Self: 'a,
            {
                let (entities, _, _) = ComponentView::split_for_iteration(self.0);
                Some(entities)
            }

            unsafe fn offset_ptrs(ptrs: Self::Ptrs, offset: usize) -> Self::Ptrs {
                ($($comp::offset_ptr(ptrs.$idx, offset),)+)
            }

            unsafe fn sparse_get<'a>(
                sparse: Self::Sparse<'a>,
                ptrs: Self::Ptrs,
                sparse_index: usize,
            ) -> Option<Self::Refs<'a>>
            where
                Self: 'a,
            {
                let index = ($(sparse.$idx.get_from_sparse(sparse_index)?,)+);
                Some(($($comp::get_from_ptr(ptrs.$idx, index.$idx),)+))
            }

            unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
            where
                Self: 'a,
            {
                ($($comp::get_from_ptr(ptrs.$idx, dense_index),)+)
            }

            fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
            where
                Self: 'a,
            {
                $(sparse.$idx.contains(entity))&&+
            }

            fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
            where
                Self: 'a,
            {
                $(!sparse.$idx.contains(entity))&&+
            }

            unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
            where
                Self: 'a,
                R: RangeBounds<usize>,
            {
                self.0.get_entities_unchecked(range)
            }

            unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slices<'a>
            where
                Self: 'a,
                R: RangeBounds<usize>,
            {
                let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
                ($(<$comp as ComponentView>::get_components_unchecked(self.$idx, bounds),)+)
            }

            unsafe fn get_entities_and_components_unchecked<'a, R>(
                self,
                range: R,
            ) -> (&'a [Entity], Self::Slices<'a>)
            where
                Self: 'a,
                R: RangeBounds<usize>,
            {
                get_entities_and_components_unchecked!(range; $(self.$idx),+)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query_part!((A, 0), (B, 1));
    impl_query_part!((A, 0), (B, 1), (C, 2));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_part!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
