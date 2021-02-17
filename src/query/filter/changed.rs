use crate::data::{ComponentFlags, Entity, SparseArray};
use crate::query::IterableView;
use std::marker::PhantomData;
use std::ops::Not;

pub struct Changed<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn changed<'a, V>(view: V) -> Changed<'a, V>
where
    V: IterableView<'a>,
{
    Changed {
        view,
        phantom: PhantomData,
    }
}

unsafe impl<'a, V> IterableView<'a> for Changed<'a, V>
where
    V: IterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn group_len(&self) -> Option<&usize> {
        V::group_len(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        if Self::get_flags(flags, index).contains(ComponentFlags::CHANGED) {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}

impl<'a, V> Not for Changed<'a, V>
where
    V: IterableView<'a>,
{
    type Output = NotChanged<'a, V>;

    fn not(self) -> Self::Output {
        NotChanged {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotChanged<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

unsafe impl<'a, V> IterableView<'a> for NotChanged<'a, V>
where
    V: IterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn group_len(&self) -> Option<&usize> {
        V::group_len(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        if !Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}
