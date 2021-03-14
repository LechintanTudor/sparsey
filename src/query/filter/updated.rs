use crate::data::{ComponentFlags, Entity, SparseVec};
use crate::query::ComponentView;
use crate::world::GroupInfo;
use std::marker::PhantomData;
use std::ops::Not;

/// `ComponentView` filter which only matches
// components updated (added or changed) this frame.
pub struct Updated<'a, V>
where
    V: ComponentView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

/// Produce a `ComponentView` filter which only matches
// components updated (added or changed) this frame.
pub fn updated<'a, V>(view: V) -> Updated<'a, V>
where
    V: ComponentView<'a>,
{
    Updated {
        view,
        phantom: PhantomData,
    }
}
unsafe impl<'a, V> ComponentView<'a> for Updated<'a, V>
where
    V: ComponentView<'a>,
{
    type Flags = V::Flags;
    type Data = V::Data;
    type Item = V::Item;

    fn group_info(&self) -> Option<GroupInfo> {
        V::group_info(&self.view)
    }

    fn split(self) -> (&'a SparseVec, &'a [Entity], Self::Flags, Self::Data) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        if Self::get_flags(flags, index).intersects(ComponentFlags::ADDED | ComponentFlags::CHANGED)
        {
            V::get_item(flags, data, index)
        } else {
            None
        }
    }
}

impl<'a, V> Not for Updated<'a, V>
where
    V: ComponentView<'a>,
{
    type Output = NotUpdated<'a, V>;

    fn not(self) -> Self::Output {
        NotUpdated {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

/// `ComponentView` filter which only matches components
// that were not updated (added or changed) this frame.
pub struct NotUpdated<'a, V>
where
    V: ComponentView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

unsafe impl<'a, V> ComponentView<'a> for NotUpdated<'a, V>
where
    V: ComponentView<'a>,
{
    type Flags = V::Flags;
    type Data = V::Data;
    type Item = V::Item;

    fn group_info(&self) -> Option<GroupInfo> {
        V::group_info(&self.view)
    }

    fn split(self) -> (&'a SparseVec, &'a [Entity], Self::Flags, Self::Data) {
        V::split(self.view)
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        if !Self::get_flags(flags, index)
            .intersects(ComponentFlags::ADDED | ComponentFlags::CHANGED)
        {
            V::get_item(flags, data, index)
        } else {
            None
        }
    }
}
