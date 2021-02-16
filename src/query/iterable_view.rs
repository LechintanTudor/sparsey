use crate::query::GroupInfo;
use crate::storage::*;

pub trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    unsafe fn group(&self) -> Option<GroupInfo>;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags;

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output>;
}
