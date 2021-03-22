use crate::data::{ComponentFlags, Entity, SparseVec};
use crate::query::ComponentView;
use crate::world::GroupInfo;
use std::marker::PhantomData;
use std::ops::Not;

/// `ComponentView` filter which only matches components added this frame.
pub struct Added<'a, V>
where
	V: ComponentView<'a>,
{
	view: V,
	phantom: PhantomData<&'a ()>,
}

/// Create a `ComponentView` filter which only matches components added this frame.
pub fn added<'a, V>(view: V) -> Added<'a, V>
where
	V: ComponentView<'a>,
{
	Added {
		view,
		phantom: PhantomData,
	}
}

unsafe impl<'a, V> ComponentView<'a> for Added<'a, V>
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
		if Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
			V::get_item(flags, data, index)
		} else {
			None
		}
	}
}

impl<'a, V> Not for Added<'a, V>
where
	V: ComponentView<'a>,
{
	type Output = NotAdded<'a, V>;

	fn not(self) -> Self::Output {
		NotAdded {
			view: self.view,
			phantom: self.phantom,
		}
	}
}

/// `ComponentView` filter which only matches components that were not added this frame.
pub struct NotAdded<'a, V>
where
	V: ComponentView<'a>,
{
	view: V,
	phantom: PhantomData<&'a ()>,
}

unsafe impl<'a, V> ComponentView<'a> for NotAdded<'a, V>
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
		if !Self::get_flags(flags, index).contains(ComponentFlags::ADDED) {
			V::get_item(flags, data, index)
		} else {
			None
		}
	}
}
