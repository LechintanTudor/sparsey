use crate::data::{Component, Entity};
use crate::world::{Comp, GroupMask};

pub unsafe trait QueryFilterComponent {
	fn mask(&self) -> GroupMask;

	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

unsafe impl QueryFilterComponent for () {
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

unsafe impl<'a, T> QueryFilterComponent for &'a Comp<'a, T>
where
	T: Component,
{
	fn mask(&self) -> GroupMask {
		match self.group_info {
			Some(info) => info.mask(),
			None => GroupMask::default(),
		}
	}

	fn includes_all(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn excludes_all(&self, entity: Entity) -> bool {
		!self.storage.contains(entity)
	}
}

pub unsafe trait QueryFilter {
	fn mask(&self) -> GroupMask;

	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}
