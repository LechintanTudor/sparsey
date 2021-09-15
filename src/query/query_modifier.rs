use crate::group::CombinedGroupInfo;
use crate::query::{IterData, UnfilteredImmutableQueryElement};
use crate::storage::{Entity, SparseArrayView};

/// Trait implemented by `QueryModifier`s.
pub unsafe trait QueryModifier<'a> {
    const ELEMENT_COUNT: usize;

    type Split;

    fn includes(&self, entity: Entity) -> bool;

    fn excludes(&self, entity: Entity) -> bool;

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>>;

    fn split(self) -> (Option<IterData<'a>>, Self::Split);

    fn includes_split(split: &Self::Split, entity: Entity) -> bool;

    fn excludes_split(split: &Self::Split, entity: Entity) -> bool;
}

unsafe impl<'a, E> QueryModifier<'a> for E
where
    E: UnfilteredImmutableQueryElement<'a>,
{
    const ELEMENT_COUNT: usize = 1;

    type Split = SparseArrayView<'a>;

    fn includes(&self, entity: Entity) -> bool {
        self.contains(entity)
    }

    fn excludes(&self, entity: Entity) -> bool {
        !self.contains(entity)
    }

    fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
        CombinedGroupInfo::default().combine(E::group_info(self)?)
    }

    fn split(self) -> (Option<IterData<'a>>, Self::Split) {
        let world_tick = self.world_tick();
        let change_tick = self.change_tick();
        let (entities, sparse) = E::split(self).into_modifier_split();

        (
            Some(IterData::new(entities, world_tick, change_tick)),
            sparse,
        )
    }

    fn includes_split(split: &Self::Split, entity: Entity) -> bool {
        split.contains(entity)
    }

    fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
        !split.contains(entity)
    }
}

macro_rules! sparse_array_view {
    ($($elem:ident),+) => {
        SparseArrayView<'a>
    };
}

macro_rules! impl_query_modifier {
    ($count:tt; $(($elem:ident, $idx:tt)),*) => {
        unsafe impl<'a, $($elem),*> QueryModifier<'a> for ($($elem,)*)
        where
            $($elem: UnfilteredImmutableQueryElement<'a>,)*
        {
            const ELEMENT_COUNT: usize = $count;

            type Split = ($(sparse_array_view!($elem),)*);

            #[allow(unused_variables)]
            fn includes(&self, entity: Entity) -> bool {
                true $(&& self.$idx.contains(entity))*
            }

            #[allow(unused_variables)]
            fn excludes(&self, entity: Entity) -> bool {
                true $(&& !self.$idx.contains(entity))*
            }

            #[allow(clippy::needless_question_mark)]
            fn group_info(&self) -> Option<CombinedGroupInfo<'a>> {
                Some(CombinedGroupInfo::default() $(.combine(self.$idx.group_info()?)?)*)
            }

            fn split(self) -> (Option<IterData<'a>>, Self::Split) {
                split_modifier!($(($elem, self.$idx)),*)
            }

            #[allow(unused_variables)]
            fn includes_split(split: &Self::Split, entity: Entity) -> bool {
                true $(&& split.$idx.contains(entity))*
            }

            #[allow(unused_variables)]
            fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
                true $(&& !split.$idx.contains(entity))*
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

    impl_query_modifier!(0; );
	impl_query_modifier!(1; (A, 0));
    impl_query_modifier!(2; (A, 0), (B, 1));
    impl_query_modifier!(3; (A, 0), (B, 1), (C, 2));
    impl_query_modifier!(4; (A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_modifier!(5; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_modifier!(6; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_modifier!(7; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_modifier!(8; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_modifier!(9; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_modifier!(10; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_modifier!(11; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_modifier!(12; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_modifier!(13; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_modifier!(14; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_modifier!(15; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_modifier!(16; (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
