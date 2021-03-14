pub use self::impls::*;

use crate::data::Entity;
use crate::query::iter::*;
use crate::query::{ComponentView, UnfilteredComponentView};
use crate::world::get_group_len;

/// Query over one or more `ComponentViews`.
pub trait Query<'a> {
    /// Item returned by `get`.
    type Item: 'a;
    /// Iterator returned by `iter`.
    type Iterator: Iterator<Item = Self::Item> + 'a;

    /// Get the components at the given `Entity`, if any.
    fn get(self, entity: Entity) -> Option<Self::Item>;

    /// Get an iterator over all components which match the `Query`.
    fn iter(self) -> Self::Iterator;

    /// Check if the views forming the `Query` are grouped (tightly packed).
    fn is_grouped(&self) -> bool;
}

/// Query over one or more `UnfilteredComponentViews`.
/// Provides functions for working with grouped components.
pub trait UnfilteredQuery<'a>
where
    Self: Query<'a>,
{
    /// Set of slices returned by `slice`.
    type SliceSet: 'a;

    /// If the components forming the `UnfilteredQuery` are grouped,
    /// return all entities which match the query.
    fn entities(self) -> Option<&'a [Entity]>;

    /// If the components forming the `UnfilteredQuery` are grouped,
    /// return ordered slices of components which match the query.
    fn slice(self) -> Option<Self::SliceSet>;

    /// If the components forming the `UnfilteredQuery` are grouped,
    /// return the entities which match the query and the ordered slices
    /// of components associated to the entities.
    fn slice_entities(self) -> Option<(&'a [Entity], Self::SliceSet)>;
}

macro_rules! impl_query {
    ($iter:ident, $(($comp:ident, $idx:tt)),+) => {
        impl<'a, $($comp),+> Query<'a> for ($($comp,)+)
        where
            $($comp: ComponentView<'a> + 'a,)+
        {
            type Item = ($($comp::Item,)+);
            type Iterator = $iter<'a, $($comp),+>;

            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some((
                    $(self.$idx.get(entity)?,)+
                ))
            }

            fn iter(self) -> Self::Iterator {
                $iter::new($(self.$idx),+)
            }

            fn is_grouped(&self) -> bool {
                (|| -> Option<_> {
                    get_group_len(&[
                        $(self.$idx.group_info()?),+
                    ])
                })()
                .is_some()
            }
        }

        impl<'a, $($comp),+> UnfilteredQuery<'a> for ($($comp,)+)
        where
            $($comp: UnfilteredComponentView<'a> + 'a,)+
        {
            type SliceSet = ($($comp::Slice,)+);

            fn entities(self) -> Option<&'a [Entity]> {
                let group_len = get_group_len(&[$(self.$idx.group_info()?),+])?;
                Some(entities!(group_len, $((self.$idx))*))
            }

            fn slice(self) -> Option<Self::SliceSet> {
                let group_len = get_group_len(&[$(self.$idx.group_info()?),+])?;

                unsafe {
                    Some((
                        $(
                            $comp::get_slice(self.$idx.split().3, group_len),
                        )+
                    ))
                }
            }

            fn slice_entities(self) -> Option<(&'a [Entity], Self::SliceSet)> {
                let group_len = get_group_len(&[$(self.$idx.group_info()?),+])?;
                Some(slice_entities!(group_len, $((self.$idx, $comp))+))
            }
        }
    };
}

macro_rules! entities {
    ($group_len:tt, ($first:expr) $(($other:expr))*) => {
        &$first.split().1[..$group_len]
    };
}

macro_rules! slice_entities {
    ($group_len:tt, ($first:expr, $first_comp:ident) $(($other:expr, $other_comp:ident))*) => {
        unsafe {
            let (_, dense, _, first_data) = $first.split();
            (
                &dense[..$group_len],
                (
                    $first_comp::get_slice(first_data, $group_len),
                    $($other_comp::get_slice($other.split().3, $group_len),)*
                )
            )
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query!(Iter1,  (A, 0));
    impl_query!(Iter2,  (A, 0), (B, 1));
    impl_query!(Iter3,  (A, 0), (B, 1), (C, 2));
    impl_query!(Iter4,  (A, 0), (B, 1), (C, 2), (D, 3));
    impl_query!(Iter5,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query!(Iter6,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query!(Iter7,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query!(Iter8,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query!(Iter9,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query!(Iter10, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query!(Iter11, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query!(Iter12, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query!(Iter13, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query!(Iter14, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query!(Iter15, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query!(Iter16, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
