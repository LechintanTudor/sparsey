use crate::data::{ComponentFlags, Entity, SparseArray};
use crate::query::ComponentView;
use crate::world::SubgroupInfo;
use std::marker::PhantomData;
use std::ops::Not;

pub struct Updated<'a, V>
where
    V: ComponentView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

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

    fn subgroup_info(&self) -> Option<SubgroupInfo> {
        V::subgroup_info(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
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

    fn subgroup_info(&self) -> Option<SubgroupInfo> {
        V::subgroup_info(&self.view)
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
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
