use crate::components::QueryGroupInfo;
use crate::query::{GetComponentUnfiltered, GetImmutableComponentUnfiltered, Passthrough};
use crate::storage::{Entity, EntitySparseArray};

pub unsafe trait QueryModifier<'a> {
    const IS_PASSTHROUGH: bool = false;

    type Sparse: 'a;

    fn includes(&self, entity: Entity) -> bool;

    fn excludes(&self, entity: Entity) -> bool;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse);

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;
}

unsafe impl<'a> QueryModifier<'a> for Passthrough {
    const IS_PASSTHROUGH: bool = true;

    type Sparse = ();

    #[inline(always)]
    fn includes(&self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes(&self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        Some(info)
    }

    #[inline(always)]
    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        Some(info)
    }

    #[inline(always)]
    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse) {
        (None, ())
    }

    #[inline(always)]
    fn includes_sparse(_sparse: &Self::Sparse, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes_sparse(_sparse: &Self::Sparse, _entity: Entity) -> bool {
        true
    }
}

unsafe impl<'a, G> QueryModifier<'a> for G
where
    G: GetImmutableComponentUnfiltered<'a>,
{
    type Sparse = &'a EntitySparseArray;

    fn includes(&self, entity: Entity) -> bool {
        GetComponentUnfiltered::get_index(self, entity).is_some()
    }

    fn excludes(&self, entity: Entity) -> bool {
        GetComponentUnfiltered::get_index(self, entity).is_none()
    }

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        info.include(GetComponentUnfiltered::group_info(self)?)
    }

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        info.exclude(GetComponentUnfiltered::group_info(self)?)
    }

    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse) {
        let (entities, sparse, _) = GetComponentUnfiltered::split(self);
        (Some(entities), sparse)
    }

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        !sparse.contains(entity)
    }
}

macro_rules! entity_sparse_array {
    ($elem:ident) => {
        &'a EntitySparseArray
    };
}

macro_rules! impl_query_modifier {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> QueryModifier<'a> for ($($elem,)+)
        where
            $($elem: GetImmutableComponentUnfiltered<'a>,)+
        {
            type Sparse = ($(entity_sparse_array!($elem),)+);

            fn includes(&self, entity: Entity) -> bool {
                $(self.$idx.get_index(entity).is_some())&&+
            }

            fn excludes(&self, entity: Entity) -> bool {
                $(self.$idx.get_index(entity).is_none())&&+
            }

            #[allow(clippy::needless_question_mark)]
            fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
                Some(info $(.include(self.$idx.group_info()?)?)+)
            }

            #[allow(clippy::needless_question_mark)]
            fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
                Some(info $(.exclude(self.$idx.group_info()?)?)+)
            }

            fn split(self) -> (Option<&'a [Entity]>, Self::Sparse) {
                split_modifier!($((self.$idx, $idx)),+)
            }

            fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
                $(sparse.$idx.contains(entity))&&+
            }

            fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
                $(!sparse.$idx.contains(entity))&&+
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query_modifier!((A, 0));
    impl_query_modifier!((A, 0), (B, 1));
    impl_query_modifier!((A, 0), (B, 1), (C, 2));
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
