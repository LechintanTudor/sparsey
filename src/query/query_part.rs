use crate::entity::{Entity, SparseVec};
use crate::query::{ComponentView, QueryGroupInfo};
use std::ops::Range;

pub unsafe trait QueryPart {
    const HAS_DATA: bool;

    type Sparse<'a>: Copy;

    type Ptrs: Copy;

    type Refs<'a>
    where
        Self: 'a;

    type Slices<'a>
    where
        Self: 'a;

    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>;

    #[must_use]
    fn contains_all(self, entity: Entity) -> bool;

    #[must_use]
    fn contains_none(self, entity: Entity) -> bool;

    #[must_use]
    fn group_info(&self) -> Option<QueryGroupInfo>;

    #[must_use]
    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a;

    #[must_use]
    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs)
    where
        Self: 'a;

    #[must_use]
    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>)
    where
        Self: 'a;

    #[must_use]
    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>;

    #[must_use]
    unsafe fn add_to_ptrs(ptrs: Self::Ptrs, index: usize) -> Self::Ptrs;

    #[must_use]
    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a>;

    #[must_use]
    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
    where
        Self: 'a;

    #[must_use]
    unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slices<'a>
    where
        Self: 'a;

    #[must_use]
    unsafe fn get_data_unchecked<'a>(self, range: Range<usize>) -> (&'a [Entity], Self::Slices<'a>)
    where
        Self: 'a;
}

#[allow(unused_variables)]
#[allow(clippy::inline_always)]
unsafe impl QueryPart for () {
    const HAS_DATA: bool = false;

    type Sparse<'a> = ();

    type Ptrs = ();

    type Refs<'a> = ();

    type Slices<'a> = ();

    #[inline(always)]
    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>> {
        None
    }

    #[inline(always)]
    fn contains_all(self, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn contains_none(self, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn group_info(&self) -> Option<QueryGroupInfo> {
        Some(QueryGroupInfo::Empty)
    }

    #[inline(always)]
    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs) {
        (&[], (), ())
    }

    #[inline(always)]
    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs) {
        (&[], ())
    }

    #[inline(always)]
    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>) {
        (&[], ())
    }

    #[inline(always)]
    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>> {
        None
    }

    #[inline(always)]
    unsafe fn add_to_ptrs(ptrs: Self::Ptrs, index: usize) -> Self::Ptrs {
        // Empty
    }

    #[inline(always)]
    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a> {
        // Empty
    }

    #[inline(always)]
    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity] {
        &[]
    }

    #[inline(always)]
    unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slices<'a> {
        // Empty
    }

    #[inline(always)]
    unsafe fn get_data_unchecked<'a>(
        self,
        range: Range<usize>,
    ) -> (&'a [Entity], Self::Slices<'a>) {
        (&[], ())
    }
}

unsafe impl<C> QueryPart for C
where
    C: ComponentView,
{
    const HAS_DATA: bool = true;

    type Sparse<'a> = &'a SparseVec;

    type Ptrs = <C as ComponentView>::Ptr;

    type Refs<'a> = <C as ComponentView>::Ref<'a> where Self: 'a;

    type Slices<'a> = <C as ComponentView>::Slice<'a> where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>> {
        ComponentView::get(self, entity)
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self, entity)
    }

    fn group_info(&self) -> Option<QueryGroupInfo> {
        Some(QueryGroupInfo::Single {
            len: ComponentView::len(self),
            group_info: ComponentView::group_info(self),
        })
    }

    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a,
    {
        ComponentView::split(self)
    }

    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, _, ptr) = ComponentView::split(self);
        (entities, ptr)
    }

    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>)
    where
        Self: 'a,
    {
        let (entities, sparse, _) = ComponentView::split(self);
        (entities, sparse)
    }

    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>> {
        sparse
            .get_sparse(sparse_index)
            .map(|dense_entity| C::get_from_ptr(ptrs, dense_entity.dense()))
    }

    unsafe fn add_to_ptrs(ptrs: Self::Ptrs, index: usize) -> Self::Ptrs {
        C::add_to_ptr(ptrs, index)
    }

    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a> {
        C::get_from_ptr(ptrs, index)
    }

    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        !sparse.contains(entity)
    }

    unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
    where
        Self: 'a,
    {
        ComponentView::get_entities_unchecked(self, range)
    }

    unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slices<'a> {
        ComponentView::get_components_unchecked(self, range)
    }

    unsafe fn get_data_unchecked<'a>(
        self,
        range: Range<usize>,
    ) -> (&'a [Entity], Self::Slices<'a>) {
        ComponentView::get_data_unchecked(self, range)
    }
}

macro_rules! impl_query_part {
    ($(($Comp:ident, $idx:tt)),+) => {
        unsafe impl<$($Comp),+> QueryPart for ($($Comp,)+)
        where
            $($Comp: ComponentView,)+
        {
            const HAS_DATA: bool = true;

            type Sparse<'a> = ($(sparse_vec!($Comp),)+);

            type Ptrs = ($($Comp::Ptr,)+);

            type Refs<'a> = ($($Comp::Ref<'a>,)+)
            where
                Self: 'a;

            type Slices<'a> = ($($Comp::Slice<'a>,)+)
            where
                Self: 'a;

            fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>> {
                Some(($(
                    self.$idx.get(entity)?,
                )+))
            }

            fn contains_all(self, entity: Entity) -> bool {
                $(
                    self.$idx.contains(entity)
                )&&+
            }

            fn contains_none(self, entity: Entity) -> bool {
                $(
                    !self.$idx.contains(entity)
                )&&+
            }

            fn group_info(&self) -> Option<QueryGroupInfo> {
                group_info!($(&self.$idx),*)
            }

            fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs)
            where
                Self: 'a,
            {
                split_sparse!($((self.$idx, $idx)),+)
            }

            fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs)
            where
                Self: 'a,
            {
                split_dense!($(self.$idx),*)
            }

            fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>)
            where
                Self: 'a,
            {
                split_filter!($(self.$idx),*)
            }

            unsafe fn get_sparse<'a>(
                sparse: Self::Sparse<'_>,
                ptrs: Self::Ptrs,
                sparse_index: usize,
            ) -> Option<Self::Refs<'a>> {
                let indexes = ($(
                    sparse.$idx.get_sparse(sparse_index)?.dense(),
                )+);

                Some(($(
                    $Comp::get_from_ptr(ptrs.$idx, indexes.$idx),
                )+))
            }

            unsafe fn add_to_ptrs(ptrs: Self::Ptrs, index: usize) -> Self::Ptrs {
                ($(
                    $Comp::add_to_ptr(ptrs.$idx, index),
                )+)
            }

            unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a> {
                ($(
                    $Comp::get_from_ptr(ptrs.$idx, index),
                )+)
            }

            fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
                $(
                    sparse.$idx.contains(entity)
                )&&+
            }

            fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
                $(
                    !sparse.$idx.contains(entity)
                )&&+
            }

            unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
            where
                Self: 'a,
            {
                self.0.get_entities_unchecked(range)
            }

            unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slices<'a>
            where
                Self: 'a,
            {
                ($(self.$idx.get_components_unchecked(range.clone()),)+)
            }

            unsafe fn get_data_unchecked<'a>(
                self,
                range: Range<usize>,
            ) -> (&'a [Entity], Self::Slices<'a>)
            where
                Self: 'a,
            {
                get_data_unchecked!(range; $(self.$idx),+)
            }
        }
    };
}

macro_rules! sparse_vec {
    ($Comp:ident) => {
        &'a SparseVec
    };
}

macro_rules! group_info {
    ($comp:expr) => {
        Some(QueryGroupInfo::Single {
            len: ComponentView::len($comp),
            group_info: ComponentView::group_info($comp),
        })
    };
    ($comp0:expr $(, $comp:expr)+) => {{
        let group_info = ComponentView::group_info($comp0)?
            $(.combine(&ComponentView::group_info($comp)?)?)+;

        Some(QueryGroupInfo::Multiple(group_info))
    }};
}

macro_rules! split_sparse {
    (($comp0:expr, $idx0:tt) $(, ($comp:expr, $idx:tt))*) => {{
        #[allow(unused_mut)]
        let (mut shortest_entities, sparse_0, ptr_0) = ComponentView::split($comp0);

        let splits = (
            (sparse_0, ptr_0),
            $({
                let (entities, sparse, ptr) = ComponentView::split($comp);

                if entities.len() < shortest_entities.len() {
                    shortest_entities = entities;
                }

                (sparse, ptr)
            },)*
        );

        (
            shortest_entities,
            (splits.0.0, $(splits.$idx.0,)*),
            (splits.0.1, $(splits.$idx.1,)*),
        )
    }};
}

macro_rules! split_dense {
    ($comp0:expr $(, $comp:expr)*) => {{
        let (entities, _, ptr_0) = ComponentView::split($comp0);
        (entities, (ptr_0, $($comp.split().2,)*))
    }};
}

macro_rules! split_filter {
    ($comp0:expr $(, $comp:expr)*) => {{
        #[allow(unused_mut)]
        let (mut shortest_entities, sparse_0, _) = ComponentView::split($comp0);

        let sparse = (
            sparse_0,
            $({
                let (entities, sparse, _) = ComponentView::split($comp);

                if entities.len() < shortest_entities.len() {
                    shortest_entities = entities;
                }

                sparse
            },)*
        );

        (shortest_entities, sparse)
    }};
}

macro_rules! get_data_unchecked {
    ($range:ident; $first:expr $(, $other:expr)*) => {{
        let (entities, first_comp) = $first.get_data_unchecked($range.clone());
        (entities, (first_comp, $($other.get_components_unchecked($range.clone()),)*))
    }};
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query_part!((A, 0));
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
