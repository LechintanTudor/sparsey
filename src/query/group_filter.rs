use crate::components::{Component, Entity};
use crate::dispatcher::{Comp, CompMut};
use crate::world::GroupMask;

pub unsafe trait GroupFilter {
	fn group_mask(&self) -> GroupMask;

	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

unsafe impl<'a, T> GroupFilter for &'a Comp<'a, T>
where
	T: Component,
{
	fn group_mask(&self) -> GroupMask {
		self.group_info.map(|g| g.mask()).unwrap_or_default()
	}

	fn includes_all(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn excludes_all(&self, entity: Entity) -> bool {
		!self.storage.contains(entity)
	}
}

unsafe impl<'a, 'b: 'a, T> GroupFilter for &'a CompMut<'b, T>
where
	T: Component,
{
	fn group_mask(&self) -> GroupMask {
		self.group_info.map(|g| g.mask()).unwrap_or_default()
	}

	fn includes_all(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn excludes_all(&self, entity: Entity) -> bool {
		!self.storage.contains(entity)
	}
}

pub unsafe trait GroupFilterComponent {
	fn mask(&self) -> GroupMask;

	fn includes(&self, entity: Entity) -> bool;

	fn excludes(&self, entity: Entity) -> bool;
}

unsafe impl<'a, T> GroupFilterComponent for &'a Comp<'a, T>
where
	T: Component,
{
	fn mask(&self) -> GroupMask {
		match self.group_info {
			Some(info) => info.mask(),
			None => GroupMask::default(),
		}
	}

	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn excludes(&self, entity: Entity) -> bool {
		!self.storage.contains(entity)
	}
}

unsafe impl<'a, 'b: 'a, T> GroupFilterComponent for &'a CompMut<'b, T>
where
	T: Component,
{
	fn mask(&self) -> GroupMask {
		match self.group_info {
			Some(info) => info.mask(),
			None => GroupMask::default(),
		}
	}

	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn excludes(&self, entity: Entity) -> bool {
		!self.storage.contains(entity)
	}
}

macro_rules! impl_filter {
	($(($comp:ident, $idx:tt)),*) => {
		unsafe impl<$($comp),*> GroupFilter for ($($comp,)*)
		where
			$($comp: GroupFilterComponent,)*
		{
			fn group_mask(&self) -> GroupMask {
				GroupMask::empty() $(| self.$idx.mask())*
			}

			#[allow(unused_variables)]
			fn includes_all(&self, entity: Entity) -> bool {
				true $(&& self.$idx.includes(entity))*
			}

			#[allow(unused_variables)]
			fn excludes_all(&self, entity: Entity) -> bool {
				true $(&& self.$idx.excludes(entity))*
			}
		}
	};
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_filter!();
	impl_filter!((A, 0));
	impl_filter!((A, 0), (B, 1));
	impl_filter!((A, 0), (B, 1), (C, 2));
	impl_filter!((A, 0), (B, 1), (C, 2), (D, 3));
	impl_filter!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
}
