use crate::data::{IterableView, UnfilteredIterableView};
use crate::group::Group;
use crate::storage::{
    ComponentFlags, Entity, SparseArray, COMPONENT_FLAG_ADDED, COMPONENT_FLAG_CHANGED,
};
use std::marker::PhantomData;
use std::ops::Not;

pub struct Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn updated<'a, V>(view: V) -> Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    Updated {
        view,
        phantom: PhantomData,
    }
}
impl<'a, V> IterableView<'a> for Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    unsafe fn group(&self) -> Option<Group> {
        V::group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        let component_flags = V::get_flags(flags, index);
        ((component_flags & COMPONENT_FLAG_ADDED) | (component_flags & COMPONENT_FLAG_CHANGED)) != 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

impl<'a, V> Not for Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
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
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotUpdated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    unsafe fn group(&self) -> Option<Group> {
        V::group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & (COMPONENT_FLAG_ADDED | COMPONENT_FLAG_CHANGED)) == 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}
