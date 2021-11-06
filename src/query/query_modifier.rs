use crate::group::CombinedGroupInfo;
use crate::query::{ImmutableUnfilteredQueryElement, Passthrough};
use crate::storage::{Entity, EntitySparseArray};

/// Trait implemented by the part of the `Query` that checks if an `Entity`
/// includes or excludes a set of components.
pub unsafe trait QueryModifier<'a> {
    const IS_PASSTHROUGH: bool;

    type Split;

    fn includes(&self, entity: Entity) -> bool;

    fn excludes(&self, entity: Entity) -> bool;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn split_modifier(self) -> (Option<&'a [Entity]>, Self::Split);

    fn includes_split(split: &Self::Split, entity: Entity) -> bool;

    fn excludes_split(split: &Self::Split, entity: Entity) -> bool;
}

unsafe impl<'a> QueryModifier<'a> for Passthrough {
    const IS_PASSTHROUGH: bool = true;

    type Split = ();

    #[inline(always)]
    fn includes(&self, _: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes(&self, _: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        Some(CombinedGroupInfo::default())
    }

    #[inline(always)]
    fn split_modifier(self) -> (Option<&'a [Entity]>, Self::Split) {
        (None, ())
    }

    #[inline(always)]
    fn includes_split(_: &Self::Split, _: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes_split(_: &Self::Split, _: Entity) -> bool {
        true
    }
}

unsafe impl<'a, E> QueryModifier<'a> for E
where
    E: ImmutableUnfilteredQueryElement<'a>,
{
    const IS_PASSTHROUGH: bool = false;

    type Split = &'a EntitySparseArray;

    #[inline]
    fn includes(&self, entity: Entity) -> bool {
        self.contains(entity, &Passthrough)
    }

    #[inline]
    fn excludes(&self, entity: Entity) -> bool {
        !self.contains(entity, &Passthrough)
    }

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        CombinedGroupInfo::default().combine(E::group_info(self)?)
    }

    fn split_modifier(self) -> (Option<&'a [Entity]>, Self::Split) {
        let (entities, sparse, _, _) = E::split(self);
        (Some(entities), sparse)
    }

    #[inline]
    fn includes_split(split: &Self::Split, entity: Entity) -> bool {
        split.contains_entity(entity)
    }

    #[inline]
    fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
        !split.contains_entity(entity)
    }
}

macro_rules! entity_sparse_array {
    ($elem:ident) => {
        &'a EntitySparseArray
    };
}

macro_rules! impl_query_modifier {
    ($(#[$attrib:meta];)* $(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> QueryModifier<'a> for ($($elem,)+)
        where
            $($elem: ImmutableUnfilteredQueryElement<'a>,)+
        {
            const IS_PASSTHROUGH: bool = false;

            type Split = ($(entity_sparse_array!($elem),)+);

            $(#[$attrib])*
            fn includes(&self, entity: Entity) -> bool {
                $(self.$idx.contains(entity, &Passthrough))&&+
            }

            $(#[$attrib])*
            fn excludes(&self, entity: Entity) -> bool {
                $(!self.$idx.contains(entity, &Passthrough))&&+
            }

            #[allow(clippy::needless_question_mark)]
            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)+)
            }

            fn split_modifier(self) -> (Option<&'a [Entity]>, Self::Split) {
                split_modifier!($((self.$idx, $idx)),+)
            }

            $(#[$attrib])*
            fn includes_split(split: &Self::Split, entity: Entity) -> bool {
                $(split.$idx.contains_entity(entity))&&+
            }

            $(#[$attrib])*
            fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
                $(!split.$idx.contains_entity(entity))&&+
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_query_modifier!(#[inline]; (A, 0));
    impl_query_modifier!(#[inline]; (A, 0), (B, 1));
    impl_query_modifier!(#[inline]; (A, 0), (B, 1), (C, 2));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
