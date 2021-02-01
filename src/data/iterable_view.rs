use crate::registry::Group;
use crate::storage::*;

pub trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    unsafe fn group(&self) -> Option<Group>;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool;

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags;

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output;
}

pub unsafe trait UnfilteredIterableView<'a>
where
    Self: IterableView<'a>,
{
}

pub(crate) unsafe fn get_output<'a, V>(
    data: V::Data,
    flags: V::Flags,
    index: usize,
) -> Option<V::Output>
where
    V: IterableView<'a>,
{
    if V::matches_flags(flags, index) {
        Some(V::get(data, flags, index))
    } else {
        None
    }
}
