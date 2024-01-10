use crate::entity::{Entity, SparseVec};
use crate::query::ComponentView;

pub unsafe trait QueryPart {
    type Sparse<'a>: Copy;

    type Ptrs: Copy;

    type Refs<'a>
    where
        Self: 'a;

    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>>;

    #[must_use]
    fn contains_all(self, entity: Entity) -> bool;

    #[must_use]
    fn contains_none(self, entity: Entity) -> bool;

    #[must_use]
    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a;

    #[must_use]
    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs)
    where
        Self: 'a;

    #[must_use]
    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>)
    where
        Self: 'a;

    #[must_use]
    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>>;

    #[must_use]
    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a>;

    #[must_use]
    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool;

    #[must_use]
    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool;
}

#[allow(unused_variables)]
#[allow(clippy::inline_always)]
unsafe impl QueryPart for () {
    type Sparse<'a> = ();

    type Ptrs = ();

    type Refs<'a> = ();

    #[inline(always)]
    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>> {
        None
    }

    #[inline(always)]
    fn contains_all(self, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn contains_none(self, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs) {
        (&[], (), ())
    }

    #[inline(always)]
    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs) {
        (&[], ())
    }

    #[inline(always)]
    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>) {
        (&[], ())
    }

    #[inline(always)]
    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>> {
        None
    }

    #[inline(always)]
    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a> {
        // Empty
    }

    #[inline(always)]
    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        true
    }
}

unsafe impl<C> QueryPart for C
where
    C: ComponentView,
{
    type Sparse<'a> = &'a SparseVec;

    type Ptrs = <C as ComponentView>::Ptr;

    type Refs<'a> = <C as ComponentView>::Ref<'a> where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Refs<'a>> {
        ComponentView::get(self, entity)
    }

    fn contains_all(self, entity: Entity) -> bool {
        ComponentView::contains(self, entity)
    }

    fn contains_none(self, entity: Entity) -> bool {
        !ComponentView::contains(self, entity)
    }

    fn split_sparse<'a>(self) -> (&'a [Entity], Self::Sparse<'a>, Self::Ptrs)
    where
        Self: 'a,
    {
        ComponentView::split(self)
    }

    fn split_dense<'a>(self) -> (&'a [Entity], Self::Ptrs)
    where
        Self: 'a,
    {
        let (entities, _, ptr) = ComponentView::split(self);
        (entities, ptr)
    }

    fn split_filter<'a>(self) -> (&'a [Entity], Self::Sparse<'a>)
    where
        Self: 'a,
    {
        let (entities, sparse, _) = ComponentView::split(self);
        (entities, sparse)
    }

    unsafe fn get_sparse<'a>(
        sparse: Self::Sparse<'_>,
        ptrs: Self::Ptrs,
        sparse_index: usize,
    ) -> Option<Self::Refs<'a>> {
        sparse
            .get_sparse(sparse_index)
            .map(|dense_entity| C::get_from_ptr(ptrs, dense_entity.dense()))
    }

    unsafe fn get_dense<'a>(ptrs: Self::Ptrs, index: usize) -> Self::Refs<'a> {
        C::get_from_ptr(ptrs, index)
    }

    fn sparse_contains_all(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        sparse.contains(entity)
    }

    fn sparse_contains_none(sparse: Self::Sparse<'_>, entity: Entity) -> bool {
        !sparse.contains(entity)
    }
}
