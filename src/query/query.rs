use crate::components::{Component, GroupInfo};
use crate::query::{Iter, QueryGroupInfo};
use crate::storage::{Entity, SparseArray};

pub unsafe trait QueryElement<'a> {
    type Item: 'a;
    type Component: Component;

    fn len(&self) -> usize;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(self, entity: Entity) -> bool;

    fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component);

    unsafe fn get_from_components_unchecked(
        components: *mut Self::Component,
        index: usize,
    ) -> Self::Item;
}

pub unsafe trait Query<'a> {
    type Item: 'a;
    type Index: Copy;
    type ComponentPtrs: Copy;
    type SparseArrays: Copy + 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn includes(self, entity: Entity) -> bool;

    fn excludes(self, entity: Entity) -> bool;

    fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs);

    fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs);

    fn split_filter(self) -> (Option<&'a [Entity]>, Self::SparseArrays);

    fn includes_split(sparse: Self::SparseArrays, entity: Entity) -> bool;

    fn excludes_split(sparse: Self::SparseArrays, entity: Entity) -> bool;

    fn get_index_from_split(sparse: Self::SparseArrays, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_components_unchecked(
        components: Self::ComponentPtrs,
        index: Self::Index,
    ) -> Self::Item;

    unsafe fn get_from_dense_components_unchecked(
        components: Self::ComponentPtrs,
        index: usize,
    ) -> Self::Item;
}

pub unsafe trait NonEmptyQuery<'a>: Query<'a> + Sized {
    // Empty
}

pub trait IntoQueryParts<'a> {
    type Get: Query<'a>;
    type Include: Query<'a> + Copy;
    type Exclude: Query<'a> + Copy;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude);
}

pub trait CompoundQuery<'a>: IntoQueryParts<'a> {
    fn get(self, entity: Entity) -> Option<<Self::Get as Query<'a>>::Item>;

    fn contains(self, entity: Entity) -> bool;
}

impl<'a, Q> CompoundQuery<'a> for Q
where
    Q: IntoQueryParts<'a>,
{
    fn get(self, entity: Entity) -> Option<<Self::Get as Query<'a>>::Item> {
        let (get, include, exclude) = self.into_query_parts();

        if exclude.excludes(entity) && include.includes(entity) {
            get.get(entity)
        } else {
            None
        }
    }

    fn contains(self, entity: Entity) -> bool {
        let (get, include, exclude) = self.into_query_parts();
        exclude.excludes(entity) && include.includes(entity) && get.includes(entity)
    }
}

pub trait IterableCompoundQuery<'a>: CompoundQuery<'a>
where
    Self::Get: NonEmptyQuery<'a>,
{
    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>;
}

impl<'a, Q> IterableCompoundQuery<'a> for Q
where
    Q: CompoundQuery<'a>,
    Q::Get: NonEmptyQuery<'a>,
{
    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude> {
        let (get, include, exclude) = self.into_query_parts();
        Iter::new(get, include, exclude)
    }
}

macro_rules! replace {
    ($from:ident, $($to:tt)+) => {
        $($to)+
    };
}

macro_rules! query_group_info {
    ($first:expr, $($other:expr),+) => {
        Some(QueryGroupInfo::Multiple($first? $(.combine($other?)?)+))
    };
}

macro_rules! split_sparse {
    (($first:expr, $_:tt), $(($other:expr, $other_idx:tt)),+) => {
        {
            let (mut entities, first_sparse, first_comp) = $first.split();

            let splits = (
                (first_sparse, first_comp),
                $(
                    {
                        let (other_entities, other_sparse, other_comp) = $other.split();

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

macro_rules! split_dense {
    ($first:expr, $($other:expr),+) => {
        {
            let (entities, _, first_comp) = $first.split();
            let comps = (first_comp, $($other.split().2),+);

            (Some(entities), comps)
        }
    };
}

macro_rules! split_filter {
    ($first:expr, $($other:expr),+) => {
        {
            let (entities, first_sparse, _) = $first.split();
            let sparse = (first_sparse, $($other.split().1),+);

            (Some(entities), sparse)
        }
    };
}

macro_rules! impl_simple_query {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> Query<'a> for ($($elem,)+)
        where
            $($elem: QueryElement<'a>,)+
        {
            type Item = ($($elem::Item,)+);
            type Index = ($(replace!($elem, usize),)+);
            type ComponentPtrs = ($(*mut $elem::Component,)+);
            type SparseArrays = ($(replace!($elem, &'a SparseArray),)+);

            fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
                query_group_info!($(self.$idx.group_info()),+)
            }

            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some((
                    $(self.$idx.get(entity)?,)+
                ))
            }

            fn includes(self, entity: Entity) -> bool {
                $(self.$idx.contains(entity))&&+
            }

            fn excludes(self, entity: Entity) -> bool {
                $(!self.$idx.contains(entity))&&+
            }

            fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs) {
                split_sparse!($((self.$idx, $idx)),+)
            }

            fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs) {
                split_dense!($(self.$idx),+)
            }

            fn split_filter(self) -> (Option<&'a [Entity]>, Self::SparseArrays) {
                split_filter!($(self.$idx),+)
            }

            fn includes_split(sparse: Self::SparseArrays, entity: Entity) -> bool {
                $(sparse.$idx.contains(entity))&&+
            }

            fn excludes_split(sparse: Self::SparseArrays, entity: Entity) -> bool {
                $(!sparse.$idx.contains(entity))&&+
            }

            fn get_index_from_split(
                sparse: Self::SparseArrays,
                entity: Entity,
            ) -> Option<Self::Index> {
                Some((
                    $(sparse.$idx.get(entity)?,)+
                ))
            }

            unsafe fn get_from_sparse_components_unchecked(
                components: Self::ComponentPtrs,
                index: Self::Index,
            ) -> Self::Item {
                ($(
                    $elem::get_from_components_unchecked(components.$idx, index.$idx),
                )+)
            }

            unsafe fn get_from_dense_components_unchecked(
                components: Self::ComponentPtrs,
                index: usize,
            ) -> Self::Item {
                ($(
                    $elem::get_from_components_unchecked(components.$idx, index),
                )+)
            }
        }

        unsafe impl<'a, $($elem),+> NonEmptyQuery<'a> for ($($elem,)+)
        where
            $($elem: QueryElement<'a>,)+
        {
            // Empty
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_simple_query!((A, 0), (B, 1));
    impl_simple_query!((A, 0), (B, 1), (C, 2));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_simple_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
