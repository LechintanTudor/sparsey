use crate::query::{ComponentView, QueryGroupInfo};
use crate::storage::{Entity, SparseArray};
use std::ops::RangeBounds;

#[doc(hidden)]
pub unsafe trait Query<'a> {
    type Item: 'a;
    type Index: Copy;
    type ComponentPtrs: Copy;
    type SparseArrays: Copy + 'a;
    type ComponentSlices: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn includes(self, entity: Entity) -> bool;

    fn excludes(self, entity: Entity) -> bool;

    fn into_any_entities(self) -> Option<&'a [Entity]>;

    fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs);

    fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs);

    fn split_filter(self) -> (Option<&'a [Entity]>, Self::SparseArrays);

    fn includes_split(sparse: Self::SparseArrays, entity: Entity) -> bool;

    fn excludes_split(sparse: Self::SparseArrays, entity: Entity) -> bool;

    fn get_index_from_split(sparse: Self::SparseArrays, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_components(
        components: Self::ComponentPtrs,
        index: Self::Index,
    ) -> Self::Item;

    unsafe fn get_from_dense_components(
        components: Self::ComponentPtrs,
        index: usize,
    ) -> Self::Item;

    unsafe fn offset_component_ptrs(
        components: Self::ComponentPtrs,
        offset: isize,
    ) -> Self::ComponentPtrs;

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>;

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlices
    where
        R: RangeBounds<usize>;

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlices)
    where
        R: RangeBounds<usize>;
}

unsafe impl<'a> Query<'a> for () {
    type Item = ();
    type Index = ();
    type ComponentPtrs = ();
    type SparseArrays = ();
    type ComponentSlices = ();

    #[inline(always)]
    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        Some(QueryGroupInfo::Empty)
    }

    #[inline(always)]
    fn get(self, _entity: Entity) -> Option<Self::Item> {
        Some(())
    }

    #[inline(always)]
    fn includes(self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes(self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn into_any_entities(self) -> Option<&'a [Entity]> {
        None
    }

    #[inline(always)]
    fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs) {
        (None, (), ())
    }

    #[inline(always)]
    fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs) {
        (None, ())
    }

    #[inline(always)]
    fn split_filter(self) -> (Option<&'a [Entity]>, Self::SparseArrays) {
        (None, ())
    }

    #[inline(always)]
    fn includes_split(_sparse: Self::SparseArrays, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes_split(_sparse: Self::SparseArrays, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn get_index_from_split(_sparse: Self::SparseArrays, _entity: Entity) -> Option<Self::Index> {
        Some(())
    }

    #[inline(always)]
    unsafe fn get_from_sparse_components(
        _components: Self::ComponentPtrs,
        _index: Self::Index,
    ) -> Self::Item {
        ()
    }

    #[inline(always)]
    unsafe fn get_from_dense_components(
        _components: Self::ComponentPtrs,
        _index: usize,
    ) -> Self::Item {
        ()
    }

    #[inline(always)]
    unsafe fn offset_component_ptrs(
        _components: Self::ComponentPtrs,
        _offset: isize,
    ) -> Self::ComponentPtrs {
        ()
    }

    #[inline(always)]
    unsafe fn get_entities_unchecked<R>(self, _range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        &[]
    }

    #[inline(always)]
    unsafe fn get_components_unchecked<R>(self, _range: R) -> Self::ComponentSlices
    where
        R: RangeBounds<usize>,
    {
        ()
    }

    #[inline(always)]
    unsafe fn get_entities_components_unchecked<R>(
        self,
        _range: R,
    ) -> (&'a [Entity], Self::ComponentSlices)
    where
        R: RangeBounds<usize>,
    {
        (&[], ())
    }
}

unsafe impl<'a, E> Query<'a> for E
where
    E: ComponentView<'a>,
{
    type Item = E::Item;
    type Index = usize;
    type ComponentPtrs = *mut E::Component;
    type SparseArrays = &'a SparseArray;
    type ComponentSlices = E::ComponentSlice;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        let len = ComponentView::len(self);
        let info = ComponentView::group_info(self);
        Some(QueryGroupInfo::Single { len, info })
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        ComponentView::get(self, entity)
    }

    fn includes(self, entity: Entity) -> bool {
        ComponentView::contains(self, entity)
    }

    fn excludes(self, entity: Entity) -> bool {
        !ComponentView::contains(self, entity)
    }

    fn into_any_entities(self) -> Option<&'a [Entity]> {
        let (entities, _, _) = ComponentView::split(self);
        Some(entities)
    }

    fn split_sparse(self) -> (Option<&'a [Entity]>, Self::SparseArrays, Self::ComponentPtrs) {
        let (entities, sparse, components) = ComponentView::split(self);
        (Some(entities), sparse, components)
    }

    fn split_dense(self) -> (Option<&'a [Entity]>, Self::ComponentPtrs) {
        let (entities, _, components) = ComponentView::split(self);
        (Some(entities), components)
    }

    fn split_filter(self) -> (Option<&'a [Entity]>, Self::SparseArrays) {
        let (entities, sparse, _) = ComponentView::split(self);
        (Some(entities), sparse)
    }

    fn includes_split(sparse: Self::SparseArrays, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn excludes_split(sparse: Self::SparseArrays, entity: Entity) -> bool {
        !sparse.contains(entity)
    }

    fn get_index_from_split(sparse: Self::SparseArrays, entity: Entity) -> Option<Self::Index> {
        sparse.get(entity)
    }

    unsafe fn get_from_sparse_components(
        components: Self::ComponentPtrs,
        index: Self::Index,
    ) -> Self::Item {
        <E as ComponentView>::get_from_component_ptr(components.add(index))
    }

    unsafe fn get_from_dense_components(
        components: Self::ComponentPtrs,
        index: usize,
    ) -> Self::Item {
        <E as ComponentView>::get_from_component_ptr(components.add(index))
    }

    unsafe fn offset_component_ptrs(
        components: Self::ComponentPtrs,
        offset: isize,
    ) -> Self::ComponentPtrs {
        components.offset(offset)
    }

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        <E as ComponentView>::get_entities_unchecked(self, range)
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlices
    where
        R: RangeBounds<usize>,
    {
        <E as ComponentView>::get_components_unchecked(self, range)
    }

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlices)
    where
        R: RangeBounds<usize>,
    {
        <E as ComponentView>::get_entities_components_unchecked(self, range)
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

            #[allow(clippy::eval_order_dependence)]
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

macro_rules! get_entities_components {
    ($range:expr; $first:expr, $($other:expr),+) => {
        {
            let bounds = ($range.start_bound().cloned(), $range.end_bound().cloned());
            let (entities, first_comp) = $first.get_entities_components_unchecked(bounds);
            (entities, (first_comp, $($other.get_components_unchecked(bounds),)+))
        }
    };
}

macro_rules! impl_query {
    ($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> Query<'a> for ($($elem,)+)
        where
            $($elem: ComponentView<'a>,)+
        {
            type Item = ($($elem::Item,)+);
            type Index = ($(replace!($elem, usize),)+);
            type ComponentPtrs = ($(*mut $elem::Component,)+);
            type SparseArrays = ($(replace!($elem, &'a SparseArray),)+);
            type ComponentSlices = ($($elem::ComponentSlice,)+);

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

            fn into_any_entities(self) -> Option<&'a [Entity]> {
                let (entities, _, _) = ComponentView::split(self.0);
                Some(entities)
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

            unsafe fn get_from_sparse_components(
                components: Self::ComponentPtrs,
                index: Self::Index,
            ) -> Self::Item {
                ($(
                    $elem::get_from_component_ptr(components.$idx.add(index.$idx)),
                )+)
            }

            unsafe fn get_from_dense_components(
                components: Self::ComponentPtrs,
                index: usize,
            ) -> Self::Item {
                ($(
                    $elem::get_from_component_ptr(components.$idx.add(index)),
                )+)
            }

            unsafe fn offset_component_ptrs(components: Self::ComponentPtrs, offset: isize) -> Self::ComponentPtrs {
                ($(
                    components.$idx.offset(offset),
                )+)
            }

            unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
            where
                R: RangeBounds<usize>,
            {
                self.0.get_entities_unchecked(range)
            }

            unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlices
            where
                R: RangeBounds<usize>,
            {
                let bounds = (range.start_bound().cloned(), range.end_bound().cloned());

                ($(
                    self.$idx.get_components_unchecked(bounds),
                )+)
            }

            unsafe fn get_entities_components_unchecked<R>(self, range: R) -> (&'a [Entity], Self::ComponentSlices)
            where
                R: RangeBounds<usize>,
            {
                get_entities_components!(range; $(self.$idx),+)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query!((A, 0), (B, 1));
    impl_query!((A, 0), (B, 1), (C, 2));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
