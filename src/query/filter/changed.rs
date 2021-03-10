use crate::data::{ComponentFlags, Entity, SparseArray};
use crate::query::ComponentView;
use crate::world::GroupInfo;
use std::marker::PhantomData;
use std::ops::Not;

pub struct Changed<'a, V>
where
    V: ComponentView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn changed<'a, V>(view: V) -> Changed<'a, V>
where
    V: ComponentView<'a>,
{
    Changed {
        view,
        phantom: PhantomData,
    }
}

unsafe impl<'a, V> ComponentView<'a> for Changed<'a, V>
where
    V: ComponentView<'a>,
{
    type Flags = V::Flags;
    type Data = V::Data;
    type Item = V::Item;

    fn group_info(&self) -> Option<GroupInfo> {
        V::group_info(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        if Self::get_flags(flags, index).contains(ComponentFlags::CHANGED) {
            V::get_item(flags, data, index)
        } else {
            None
        }
    }
}

impl<'a, V> Not for Changed<'a, V>
where
    V: ComponentView<'a>,
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
    V: ComponentView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

unsafe impl<'a, V> ComponentView<'a> for NotChanged<'a, V>
where
    V: ComponentView<'a>,
{
    type Flags = V::Flags;
    type Data = V::Data;
    type Item = V::Item;

    fn group_info(&self) -> Option<GroupInfo> {
        V::group_info(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        if !Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
            V::get_item(flags, data, index)
        } else {
            None
        }
    }
}
