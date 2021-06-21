use crate::components::{Component, ComponentStorage, Entity, Ticks};
use crate::layout::Layout;
use crate::world::{
	BorrowStorages, ComponentSet, ComponentStorages, EntityStorage, NoSuchEntity, TickOverflow,
};
use std::any::TypeId;

/// Container for component storages and entities.
#[derive(Default)]
pub struct World {
	entities: EntityStorage,
	components: ComponentStorages,
	tick: Ticks,
}

impl World {
	pub fn with_layout(layout: &Layout) -> Self {
		let mut world = Self::default();
		world.set_layout(layout);
		world
	}

	pub fn set_layout(&mut self, layout: &Layout) {
		self.components.set_layout(layout, self.entities.as_ref());
	}

	pub fn register<T>(&mut self)
	where
		T: Component,
	{
		self.components.register::<T>()
	}

	pub unsafe fn register_storage(&mut self, component: TypeId, storage: ComponentStorage) {
		self.components.register_storage(component, storage);
	}

	/// Create an `Entity` with the given components and return it.
	pub fn create<C>(&mut self, components: C) -> Entity
	where
		C: ComponentSet,
	{
		let entity = self.entities.create();
		let _ = self.insert(entity, components);
		entity
	}

	/// Extend the `World` with a component iterator.
	/// Return the newly created entities as a slice.
	pub fn extend<C, I>(&mut self, components_iter: I) -> &[Entity]
	where
		C: ComponentSet,
		I: IntoIterator<Item = C>,
	{
		let initial_entity_count = self.entities.as_ref().len();

		let families = {
			let (mut storages, families) = C::Storages::borrow_with_families(&self.components);
			let entities = &mut self.entities;
			let tick = self.tick;

			components_iter.into_iter().for_each(|components| {
				let entity = entities.create();

				unsafe {
					C::insert(&mut storages, entity, components, tick);
				}
			});

			families
		};

		let new_entities = &self.entities.as_ref()[initial_entity_count..];

		for i in families.indexes() {
			for &entity in new_entities {
				unsafe {
					self.components.grouped.group_components(i, entity);
				}
			}
		}

		new_entities
	}

	/// Destroy an `Entity` and all of its components.
	/// Return whether or not there was an `Entity` to destroy.
	pub fn destroy(&mut self, entity: Entity) -> bool {
		if !self.entities.destroy(entity) {
			return false;
		}

		for i in 0..self.components.grouped.group_family_count() {
			unsafe {
				self.components.grouped.ungroup_components(i, entity);
			}
		}

		for storage in self.components.iter_storages_mut() {
			storage.remove_and_drop(entity);
		}

		true
	}

	/// Insert a set of `Components` to the given `Entity`, if it exists.
	pub fn insert<C>(&mut self, entity: Entity, components: C) -> Result<(), NoSuchEntity>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return Err(NoSuchEntity);
		}

		let families = unsafe {
			let (mut storages, families) = C::Storages::borrow_with_families(&self.components);
			C::insert(&mut storages, entity, components, self.tick);
			families
		};

		for i in families.indexes() {
			unsafe {
				self.components.grouped.group_components(i, entity);
			}
		}

		Ok(())
	}

	/// Remove a set of `Components` from an `Entity` and return them if they
	/// were all present before calling this function.
	pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return None;
		}

		let families = C::Storages::families(&self.components);

		for i in families.indexes() {
			unsafe {
				self.components.grouped.ungroup_components(i, entity);
			}
		}

		unsafe {
			let mut storages = C::Storages::borrow(&self.components);
			C::remove(&mut storages, entity)
		}
	}

	/// Delete a set of components from an `Entity`.
	pub fn delete<C>(&mut self, entity: Entity)
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return;
		}

		let families = C::Storages::families(&self.components);

		for i in families.indexes() {
			unsafe {
				self.components.grouped.ungroup_components(i, entity);
			}
		}

		unsafe {
			let mut storages = C::Storages::borrow(&self.components);
			C::delete(&mut storages, entity);
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.entities.contains(entity)
	}

	pub fn clear(&mut self) {
		self.entities.clear();
		self.components.clear();
	}

	pub fn advance_ticks(&mut self) -> Result<(), TickOverflow> {
		if self.tick != Ticks::MAX {
			self.tick += 1;
			Ok(())
		} else {
			self.tick = 0;
			Err(TickOverflow)
		}
	}

	pub fn entities(&self) -> &[Entity] {
		self.entities.as_ref()
	}

	pub(crate) fn entity_storage(&self) -> &EntityStorage {
		&self.entities
	}

	pub(crate) fn component_storages(&self) -> &ComponentStorages {
		&self.components
	}

	pub(crate) fn tick(&self) -> Ticks {
		self.tick
	}

	pub(crate) fn maintain(&mut self) {
		self.entities.maintain();
	}
}
