use crate::query::{
    ImmutableUnfilteredQueryElement, IntoQueryParts, InvalidGroup, Passthrough, QueryBase,
};
use crate::storage::Entity;
use crate::{query, utils};
use std::ops::RangeBounds;

/// Trait used for slicing queries with grouped component storages.
pub unsafe trait SliceQuery<'a>
where
    Self: IntoQueryParts<'a, Filter = Passthrough>,
    Self::Base: SliceQueryBase<'a>,
{
    /// Returns a slice containing all the entities that match the `Query`.
    fn entities(self) -> Result<&'a [Entity], InvalidGroup>;

    /// Returns a tuple of slices containing all components that match the
    /// `Query`.
    fn components(self) -> Result<<Self::Base as SliceQueryBase<'a>>::Slices, InvalidGroup>;

    /// Returns all entities and components that match the `Query`.
    fn entities_components(
        self,
    ) -> Result<(&'a [Entity], <Self::Base as SliceQueryBase<'a>>::Slices), InvalidGroup>;
}

unsafe impl<'a, Q> SliceQuery<'a> for Q
where
    Q: IntoQueryParts<'a, Filter = Passthrough>,
    Q::Base: SliceQueryBase<'a>,
{
    fn entities(self) -> Result<&'a [Entity], InvalidGroup> {
        if query::is_trivial_group::<Q::Base, Q::Include, Q::Exclude>() {
            let (base, _, _, _) = self.into_query_parts();
            Ok(unsafe { base.get_entities_unchecked(..) })
        } else {
            let (base, include, exclude, _) = self.into_query_parts();
            let range = query::group_range(&base, &include, &exclude)?;
            Ok(unsafe { base.get_entities_unchecked(range) })
        }
    }

    fn components(self) -> Result<<Self::Base as SliceQueryBase<'a>>::Slices, InvalidGroup> {
        if query::is_trivial_group::<Q::Base, Q::Include, Q::Exclude>() {
            let (base, _, _, _) = self.into_query_parts();
            Ok(unsafe { base.get_components_unchecked(..) })
        } else {
            let (base, include, exclude, _) = self.into_query_parts();
            let range = query::group_range(&base, &include, &exclude)?;
            Ok(unsafe { base.get_components_unchecked(range) })
        }
    }

    fn entities_components(
        self,
    ) -> Result<(&'a [Entity], <Self::Base as SliceQueryBase<'a>>::Slices), InvalidGroup> {
        if query::is_trivial_group::<Q::Base, Q::Include, Q::Exclude>() {
            let (base, _, _, _) = self.into_query_parts();
            Ok(unsafe { base.get_entities_components_unchecked(..) })
        } else {
            let (base, include, exclude, _) = self.into_query_parts();
            let range = query::group_range(&base, &include, &exclude)?;
            Ok(unsafe { base.get_entities_components_unchecked(range) })
        }
    }
}

/// Trait used by `QuerySlice` to get component slices from `QueryBase`.
pub unsafe trait SliceQueryBase<'a>
where
    Self: QueryBase<'a>,
{
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

unsafe impl<'a, E> SliceQueryBase<'a> for E
where
    E: ImmutableUnfilteredQueryElement<'a>,
{
    type Slices = &'a [E::Component];

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        ImmutableUnfilteredQueryElement::entities(&self)
            .get_unchecked(utils::range_to_bounds(&range))
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::Slices
    where
        R: RangeBounds<usize>,
    {
        ImmutableUnfilteredQueryElement::components(&self)
            .get_unchecked(utils::range_to_bounds(&range))
    }

    unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::Slices)
    where
        R: RangeBounds<usize>,
    {
        let entities = ImmutableUnfilteredQueryElement::entities(&self)
            .get_unchecked(utils::range_to_bounds(&range));

        let components = ImmutableUnfilteredQueryElement::components(&self)
            .get_unchecked(utils::range_to_bounds(&range));

        (entities, components)
    }
}

macro_rules! impl_slice_query_base {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> SliceQueryBase<'a> for ($($elem,)+)
        where
            $($elem: ImmutableUnfilteredQueryElement<'a>,)+
        {
            type Slices = ($(&'a [$elem::Component],)+);

            unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
            where
                R: RangeBounds<usize>,
            {
                ImmutableUnfilteredQueryElement::entities(&self.0)
                    .get_unchecked(utils::range_to_bounds(&range))
            }

            unsafe fn get_components_unchecked<R>(self, range: R) -> Self::Slices
            where
                R: RangeBounds<usize>,
            {
                ($(
                    ImmutableUnfilteredQueryElement::components(&self.$idx)
                        .get_unchecked(utils::range_to_bounds(&range))
                ,)+)
            }

            unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::Slices)
            where
                R: RangeBounds<usize>,
            {
                let entities = ImmutableUnfilteredQueryElement::entities(&self.0)
                    .get_unchecked(utils::range_to_bounds(&range));

                let components = ($(
                    ImmutableUnfilteredQueryElement::components(&self.$idx)
                        .get_unchecked(utils::range_to_bounds(&range))
                ,)+);

                (entities, components)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_slice_query_base!((A, 0));
    impl_slice_query_base!((A, 0), (B, 1));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
