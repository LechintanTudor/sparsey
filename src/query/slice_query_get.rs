use crate::query::{GetImmutableComponentUnfiltered, QueryGet};
use crate::storage::Entity;
use crate::utils::range_to_bounds;
use std::ops::RangeBounds;

/// Trait used for getting `Component` and `Entity` slices from grouped components.
pub unsafe trait SliceQueryGet<'a>: QueryGet<'a> {
    /// Component slices returned by slicing the `Query`.
    type Slices;

    /// # Safety
    /// The `Query` must be safely indexable by `range`.
    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>;

    /// # Safety
    /// The `Query` must be safely indexable by `range`.
    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::Slices
    where
        R: RangeBounds<usize>;

    /// # Safety
    /// The `Query` must be safely indexable by `range`.
    unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::Slices)
    where
        R: RangeBounds<usize>;
}

unsafe impl<'a, C> SliceQueryGet<'a> for C
where
    C: GetImmutableComponentUnfiltered<'a>,
{
    type Slices = &'a [C::Component];

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        GetImmutableComponentUnfiltered::entities(&self).get_unchecked(range_to_bounds(&range))
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::Slices
    where
        R: RangeBounds<usize>,
    {
        GetImmutableComponentUnfiltered::components(&self).get_unchecked(range_to_bounds(&range))
    }

    unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::Slices)
    where
        R: RangeBounds<usize>,
    {
        (
            GetImmutableComponentUnfiltered::entities(&self).get_unchecked(range_to_bounds(&range)),
            GetImmutableComponentUnfiltered::components(&self)
                .get_unchecked(range_to_bounds(&range)),
        )
    }
}

macro_rules! impl_slice_query_get {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> SliceQueryGet<'a> for ($($elem,)+)
        where
            $($elem: GetImmutableComponentUnfiltered<'a>,)+
        {
            type Slices = ($(&'a [$elem::Component],)+);

            unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
            where
                R: RangeBounds<usize>,
            {
                &self.0.entities()[range_to_bounds(&range)]
            }

            unsafe fn get_components_unchecked<R>(self, range: R) -> Self::Slices
            where
                R: RangeBounds<usize>,
            {
                ($(
                    &self.$idx.components()[range_to_bounds(&range)],
                )+)
            }

            unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::Slices)
            where
                R: RangeBounds<usize>,
            {
                (
                    &self.0.entities()[range_to_bounds(&range)],
                    ($(
                        &self.$idx.components()[range_to_bounds(&range)],
                    )+)
                )
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_slice_query_get!((A, 0));
    impl_slice_query_get!((A, 0), (B, 1));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_slice_query_get!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
