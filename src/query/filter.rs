use crate::data::{Component, Entity};
use crate::world::{Comp, GroupMask};

pub unsafe trait GroupFilter {
	fn mask(&self) -> GroupMask;

	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

unsafe impl GroupFilter for () {
	fn mask(&self) -> GroupMask {
		GroupMask::default()
	}

	fn includes_all(&self, _: Entity) -> bool {
		true
	}

	fn excludes_all(&self, _: Entity) -> bool {
		true
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

macro_rules! impl_filter {
	($(($comp:ident, $idx:tt)),+) => {
		unsafe impl<$($comp),+> GroupFilter for ($($comp,)+)
		where
			$($comp: GroupFilterComponent,)+
		{
			fn mask(&self) -> GroupMask {
				$(self.$idx.mask())|+
			}

			fn includes_all(&self, entity: Entity) -> bool {
				$(self.$idx.includes(entity))&&+
			}

			fn excludes_all(&self, entity: Entity) -> bool {
				$(self.$idx.excludes(entity))&&+
			}
		}
	};
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_filter!((A, 0));
	impl_filter!((A, 0), (B, 1));
	impl_filter!((A, 0), (B, 1), (C, 2));
	impl_filter!((A, 0), (B, 1), (C, 2), (D, 3));
	impl_filter!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
}
