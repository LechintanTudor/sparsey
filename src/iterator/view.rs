use crate::{Entity, SparseArray, SparseSet};

pub trait SparseSetLike<'a> {
    type Ref: 'a;
    type Slice: 'a + Copy;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice);

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref;
}

impl<'a, T> SparseSetLike<'a> for &'a SparseSet<T> {
    type Ref = &'a T;
    type Slice = *const T;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice) {
        let (sparse, dense, data) = self.split();
        (sparse, dense, data.as_ptr())
    }

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref {
        &*values.add(entity.index())
    }
}

impl<'a, T> SparseSetLike<'a> for &'a mut SparseSet<T> {
    type Ref = &'a mut T;
    type Slice = *mut T;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice) {
        let (sparse, dense, data) = self.split_mut();
        (sparse, dense, data.as_mut_ptr())
    }

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref {
        &mut *values.add(entity.index())
    }
}

pub trait View<'a> {
    const STRICT: bool;
    type SparseSet: SparseSetLike<'a>;
    type Output: 'a;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output>;
}

impl<'a, T> View<'a> for &'a T {
    const STRICT: bool = true;
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> View<'a> for &'a mut T {
    const STRICT: bool = true;
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> View<'a> for Option<&'a T> {
    const STRICT: bool = false;
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

impl<'a, T> View<'a> for Option<&'a mut T> {
    const STRICT: bool = false;
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

pub unsafe fn fetch<'a, T>(
    sparse: &SparseArray,
    values: <T::SparseSet as SparseSetLike<'a>>::Slice,
    entity: Entity,
) -> Option<T::Output>
where
    T: View<'a>,
{
    T::fetch(
        sparse
            .get_valid(entity)
            .map(|&e| <T::SparseSet as SparseSetLike<'a>>::fetch(values, e)),
    )
}
