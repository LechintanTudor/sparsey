use crate::data::IterableView;
use crate::storage::{ComponentFlags, Entity, SparseArray};
use crate::world::GroupInfo;
use std::marker::PhantomData;
use std::ops::Not;

pub struct Added<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn added<'a, V>(view: V) -> Added<'a, V>
where
    V: IterableView<'a>,
{
    Added {
        view,
        phantom: PhantomData,
    }
}

impl<'a, V> IterableView<'a> for Added<'a, V>
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
        if Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}

impl<'a, V> Not for Added<'a, V>
where
    V: IterableView<'a>,
{
    type Output = NotAdded<'a, V>;

    fn not(self) -> Self::Output {
        NotAdded {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotAdded<'a, V>
where
    V: IterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotAdded<'a, V>
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
        if !Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
            V::get(data, flags, index)
        } else {
            None
        }
    }
}
