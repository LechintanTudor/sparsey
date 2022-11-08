use crate::query::ComponentView;
use crate::storage::Entity;

pub trait QueryPart {
    type Refs<'a>
    where
        Self: 'a;

    type Slices<'a>
    where
        Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a;

    fn contains_all(self, entity: Entity) -> bool;

    fn contains_none(self, entity: Entity) -> bool;
}

macro_rules! impl_query_part {
    ($(($comp:ident, $idx:tt)),+) => {
        impl<$($comp),+> QueryPart for ($($comp,)+)
        where
            $($comp: ComponentView,)+
        {
            type Refs<'a> = ($($comp::Ref<'a>,)+) where Self: 'a;
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
        }
    };
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
