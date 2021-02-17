use crate::data::{ComponentFlags, Entity, SparseArray};

pub unsafe trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    fn group_len(&self) -> Option<&usize>;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags;

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output>;
}
