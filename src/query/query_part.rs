use crate::query::ComponentView;
use crate::storage::{Entity, SparseArray};

pub trait QueryPart {
    type Refs<'a>
    where
        Self: 'a;

    type Sparse<'a>: Copy
    where
        Self: 'a;

    type Ptrs: Copy;

    type Slices<'a>
    where
        Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a;

    fn contains_all(self, entity: Entity) -> bool;

    fn contains_none(self, entity: Entity) -> bool;

    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a;

    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a;

    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a;

    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a;
}

impl QueryPart for () {
    type Refs<'a> = ();
    type Sparse<'a> = ();
    type Ptrs = ();
    type Slices<'a> = ();

    fn get<'a>(self, _entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        None
    }

    fn contains_all(self, _entity: Entity) -> bool {
        true
    }

    fn contains_none(self, _entity: Entity) -> bool {
        true
    }

    unsafe fn sparse_get<'a>(
        _sparse: Self::Sparse<'a>,
        _ptrs: Self::Ptrs,
        _sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        None
    }

    unsafe fn dense_get<'a>(_ptrs: Self::Ptrs, _dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        ()
    }

    fn sparse_contains_all<'a>(_sparse: Self::Sparse<'a>, _entity: Entity) -> bool
    where
        Self: 'a,
    {
        true
    }

    fn sparse_contains_none<'a>(_sparse: Self::Sparse<'a>, _entity: Entity) -> bool
    where
        Self: 'a,
    {
        true
    }
}

impl<C> QueryPart for C
where
    C: ComponentView,
{
    type Refs<'a> = C::Ref<'a> where Self: 'a;
    type Sparse<'a> = &'a SparseArray where Self: 'a;
    type Ptrs = C::Ptr;
    type Slices<'a> = C::Slice<'a> where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        ComponentView::get(self, entity)
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self, entity)
    }

    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        let dense_index = sparse.get_from_sparse(sparse_index)?;
        Some(<C as ComponentView>::get_from_ptr(ptrs, dense_index))
    }

    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        <C as ComponentView>::get_from_ptr(ptrs, dense_index)
    }

    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        sparse.contains(entity)
    }

    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        !sparse.contains(entity)
    }
}

impl<C> QueryPart for (C,)
where
    C: ComponentView,
{
    type Refs<'a> = (C::Ref<'a>,) where Self: 'a;
    type Sparse<'a> = (&'a SparseArray,) where Self: 'a;
    type Ptrs = (C::Ptr,);
    type Slices<'a> = (C::Slice<'a>,) where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        Some((ComponentView::get(self.0, entity)?,))
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self.0, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self.0, entity)
    }

    unsafe fn sparse_get<'a>(
        sparse: Self::Sparse<'a>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>
    where
        Self: 'a,
    {
        let dense_index = sparse.0.get_from_sparse(sparse_index)?;
        Some((<C as ComponentView>::get_from_ptr(ptrs.0, dense_index),))
    }

    unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
    where
        Self: 'a,
    {
        (<C as ComponentView>::get_from_ptr(ptrs.0, dense_index),)
    }

    fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        sparse.0.contains(entity)
    }

    fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
    where
        Self: 'a,
    {
        !sparse.0.contains(entity)
    }
}

macro_rules! replace_with_sparse_array_ref {
    ($old:tt) => {
        &'a SparseArray
    };
}

macro_rules! impl_query_part {
    ($(($comp:ident, $idx:tt)),+) => {
        impl<$($comp),+> QueryPart for ($($comp,)+)
        where
            $($comp: ComponentView,)+
        {
            type Refs<'a> = ($($comp::Ref<'a>,)+) where Self: 'a;
            type Sparse<'a> = ($(replace_with_sparse_array_ref!($comp),)+) where Self: 'a;
            type Ptrs = ($($comp::Ptr,)+);
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

            unsafe fn sparse_get<'a>(
                sparse: Self::Sparse<'a>,
                ptrs: Self::Ptrs,
                sparse_index: usize,
            ) -> Option<Self::Refs<'a>>
            where
                Self: 'a,
            {
                let index = ($(sparse.$idx.get_from_sparse(sparse_index)?,)+);
                Some(($($comp::get_from_ptr(ptrs.$idx, index.$idx),)+))
            }

            unsafe fn dense_get<'a>(ptrs: Self::Ptrs, dense_index: usize) -> Self::Refs<'a>
            where
                Self: 'a,
            {
                ($($comp::get_from_ptr(ptrs.$idx, dense_index),)+)
            }

            fn sparse_contains_all<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
            where
                Self: 'a,
            {
                $(sparse.$idx.contains(entity))&&+
            }

            fn sparse_contains_none<'a>(sparse: Self::Sparse<'a>, entity: Entity) -> bool
            where
                Self: 'a,
            {
                $(!sparse.$idx.contains(entity))&&+
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

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
