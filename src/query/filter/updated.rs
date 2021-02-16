use crate::query::{GroupInfo, IterableView};
use crate::storage::{ComponentFlags, Entity, SparseArray};
use std::marker::PhantomData;
use std::ops::Not;

pub struct Updated<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn updated<'a, V>(view: V) -> Updated<'a, V>
where
    V: IterableView<'a>,
{
    Updated {
        view,
        phantom: PhantomData,
    }
}
impl<'a, V> IterableView<'a> for Updated<'a, V>
where
    V: IterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    unsafe fn group(&self) -> Option<GroupInfo> {
        V::group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        if Self::get_flags(flags, index).intersects(ComponentFlags::ADDED | ComponentFlags::CHANGED)
        {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}

impl<'a, V> Not for Updated<'a, V>
where
    V: IterableView<'a>,
{
    type Output = NotUpdated<'a, V>;

    fn not(self) -> Self::Output {
        NotUpdated {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotUpdated<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotUpdated<'a, V>
where
    V: IterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    unsafe fn group(&self) -> Option<GroupInfo> {
        V::group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        if !Self::get_flags(flags, index)
            .intersects(ComponentFlags::ADDED | ComponentFlags::CHANGED)
        {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}
