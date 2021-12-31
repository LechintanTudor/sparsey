use crate::components::{Component, Group, StorageMask};
use crate::storage::{Entity, EntitySparseArray};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone, Copy)]
pub struct GroupInfo<'a> {
    group: NonNull<Group>,
    offset: u32,
    mask: StorageMask,
    _phantom: PhantomData<&'a [Group]>,
}

impl<'a> GroupInfo<'a> {
    pub fn combine(self, info: Self) -> Option<Self> {
        if self.group != info.group {
            return None;
        }

        Some(Self {
            group: self.group,
            offset: self.offset.max(info.offset),
            mask: self.mask | info.mask,
            _phantom: PhantomData,
        })
    }
}

pub unsafe trait SimpleQueryElement<'a> {
    type Item: 'a;
    type Component: Component;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(self, entity: Entity) -> bool;

    fn split(self) -> (&'a [Entity], &'a EntitySparseArray, *mut Self::Component);

    unsafe fn get_from_components_unchecked(
        components: *mut Self::Component,
        index: usize,
    ) -> Self::Item;
}

pub unsafe trait SimpleQuery<'a> {
    type Item: 'a;
    type Index: Copy;
    type ComponentPtrs: Copy;
    type SparseArrays: Copy + 'a;

    fn group_info(&self) -> Option<Option<GroupInfo<'a>>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn includes(self, entity: Entity) -> bool;

    fn excludes(self, entity: Entity) -> bool;

    fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs);

    fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs);

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

pub trait IntoQueryParts<'a> {
    type Get: SimpleQuery<'a>;
    type Include: SimpleQuery<'a> + Copy;
    type Exclude: SimpleQuery<'a> + Copy;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude);
}

pub trait Query<'a>: IntoQueryParts<'a> {
    fn get(self, entity: Entity) -> Option<<Self::Get as SimpleQuery<'a>>::Item>;

    fn contains(self, entity: Entity) -> bool;
}

impl<'a, Q> Query<'a> for Q
where
    Q: IntoQueryParts<'a>,
{
    fn get(self, entity: Entity) -> Option<<Self::Get as SimpleQuery<'a>>::Item> {
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

macro_rules! replace {
    ($from:ident, $($to:tt)+) => {
        $($to)+
    };
}

macro_rules! impl_simple_query {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> SimpleQuery<'a> for ($($elem,)+)
        where
            $($elem: SimpleQueryElement<'a>,)+
        {
            type Item = ($($elem::Item,)+);
            type Index = ($(replace!($elem, usize),)+);
            type ComponentPtrs = ($(*mut $elem::Component,)+);
            type SparseArrays = ($(replace!($elem, &'a EntitySparseArray),)+);

            fn group_info(&self) -> Option<Option<GroupInfo<'a>>> {
                todo!()
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

            fn split_sparse(
                self,
            ) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs) {
                todo!()
            }

            fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs) {
                todo!()
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
