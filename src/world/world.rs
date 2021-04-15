use crate::components::{Component, Entity};
use crate::world::{Comp, CompMut, ComponentStorages, EntityStorage};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

/// Container for component storages and entities.
#[derive(Default)]
pub struct World {
	entities: EntityStorage,
	components: ComponentStorages,
	group_indexes: HashSet<usize>,
}

impl World {
	pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
	where
		T: Component,
	{
		self.components.borrow_comp::<T>()
	}

	pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
	where
		T: Component,
	{
		self.components.borrow_comp_mut::<T>()
	}

	pub(crate) fn entity_storage(&self) -> &EntityStorage {
		&self.entities
	}
}

/// Error returned when trying to access entities
/// which are not contained in the `World`.
#[derive(Debug)]
pub struct NoSuchEntity;

impl Error for NoSuchEntity {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

impl fmt::Display for NoSuchEntity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "No such entity was found in the World")
	}
}
