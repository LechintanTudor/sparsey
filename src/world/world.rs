use crate::components::{Component, ComponentStorage, Entity, NonZeroTicks, Ticks};
use crate::layout::Layout;
use crate::world::{
	BorrowStorages, ComponentSet, ComponentStorages, EntityStorage, NoSuchEntity, TickOverflow,
};
use std::any::TypeId;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WorldId(NonZeroU64);

impl WorldId {
	fn new() -> Self {
		static COUNTER: AtomicU64 = AtomicU64::new(1);

		let id = COUNTER.fetch_add(1, Ordering::Relaxed);
		NonZeroU64::new(id).map(Self).expect("Ran out of WorldIds")
	}
}

/// Container for component storages and entities.
pub struct World {
	id: WorldId,
	tick: NonZeroTicks,
	entities: EntityStorage,
	storages: ComponentStorages,
}

impl Default for World {
	fn default() -> Self {
		Self {
			id: WorldId::new(),
			tick: NonZeroTicks::new(1).unwrap(),
			entities: Default::default(),
			storages: Default::default(),
		}
	}
}

impl World {
	/// Creates an empty world with the storages arranged as described by
	/// `layout`.
	pub fn with_layout(layout: &Layout) -> Self {
		let mut world = Self::default();
		world.set_layout(layout);
		world
	}

	/// Arranges the storages as described by `layout`. This function iterates
	/// through all the entities to ararange their components, so it is best
	/// called right after creating the `World`.
	pub fn set_layout(&mut self, layout: &Layout) {
		self.storages.set_layout(layout, self.entities.as_ref());
	}

	/// Creates a component storage for `T` if one doesn't already exist.
	pub fn register<T>(&mut self)
	where
		T: Component,
	{
		self.storages.register::<T>()
	}

	/// Creates an `Entity` with the given `components` and returns it.
	pub fn create<C>(&mut self, components: C) -> Entity
	where
		C: ComponentSet,
	{
		let entity = self.entities.create();
		let _ = self.insert(entity, components);
		entity
	}

	/// Creates new `Entities` with the component sets yielded by
	/// `components_iter`. Returns the newly created entities as a `slice`.
	pub fn extend<C, I>(&mut self, components_iter: I) -> &[Entity]
	where
		C: ComponentSet,
		I: IntoIterator<Item = C>,
	{
		let initial_entity_count = self.entities.as_ref().len();

		let families = {
			let (mut storages, families) = C::Storages::borrow_with_families(&self.storages);
			let entities = &mut self.entities;
			let tick = self.tick.get();

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
					self.storages.grouped.group_components(i, entity);
				}
			}
		}

		new_entities
	}

	/// Removes `entity` and all of its `components` from the world.
	/// Returns `true` if `entity` was contained in the world before the call.
	pub fn destroy(&mut self, entity: Entity) -> bool {
		if !self.entities.destroy(entity) {
			return false;
		}

		for i in 0..self.storages.grouped.group_family_count() {
			unsafe {
				self.storages.grouped.ungroup_components(i, entity);
			}
		}

		for storage in self.storages.iter_storages_mut() {
			storage.remove_and_drop(entity);
		}

		true
	}

	/// Inserts `components` to `entity` if `entity` is contained in the world.
	pub fn insert<C>(&mut self, entity: Entity, components: C) -> Result<(), NoSuchEntity>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return Err(NoSuchEntity);
		}

		let families = unsafe {
			let (mut storages, families) = C::Storages::borrow_with_families(&self.storages);
			C::insert(&mut storages, entity, components, self.tick.get());
			families
		};

		for i in families.indexes() {
			unsafe {
				self.storages.grouped.group_components(i, entity);
			}
		}

		Ok(())
	}

	/// Removes a component set from `entity` and returns them if they were all
	/// contained in the world before the call.
	pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return None;
		}

		let families = C::Storages::families(&self.storages);

		for i in families.indexes() {
			unsafe {
				self.storages.grouped.ungroup_components(i, entity);
			}
		}

		unsafe {
			let mut storages = C::Storages::borrow(&self.storages);
			C::remove(&mut storages, entity)
		}
	}

	/// Deletes a component set from `entity`. This is faster than `removing`
	/// the components.
	pub fn delete<C>(&mut self, entity: Entity)
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return;
		}

		let families = C::Storages::families(&self.storages);

		for i in families.indexes() {
			unsafe {
				self.storages.grouped.ungroup_components(i, entity);
			}
		}

		unsafe {
			let mut storages = C::Storages::borrow(&self.storages);
			C::delete(&mut storages, entity);
		}
	}

	/// Returns `true` if `entity` is contained in the world.
	pub fn contains(&self, entity: Entity) -> bool {
		self.entities.contains(entity)
	}

	/// Removes all `entities` and `components` in the world.
	pub fn clear(&mut self) {
		self.entities.clear();
		self.storages.clear();
	}

	/// Advances the `current world tick`. Should be called after each game
	/// update.
	pub fn advance_ticks(&mut self) -> Result<(), TickOverflow> {
		if self.tick.get() != Ticks::MAX {
			self.tick = NonZeroTicks::new(self.tick.get() + 1).unwrap();
			Ok(())
		} else {
			self.tick = NonZeroTicks::new(1).unwrap();
			Err(TickOverflow)
		}
	}

	/// Returns all the `entities` in the world as a `slice`.
	pub fn entities(&self) -> &[Entity] {
		self.entities.as_ref()
	}

	/// Returns the `id` which uniquely identifies this `World`.
	pub fn id(&self) -> WorldId {
		self.id
	}

	/// Returns the `current world tick` used for `change detection`.
	pub fn tick(&self) -> Ticks {
		self.tick.get()
	}

	pub(crate) unsafe fn register_storage(&mut self, component: TypeId, storage: ComponentStorage) {
		self.storages.register_storage(component, storage);
	}

	pub(crate) fn entity_storage(&self) -> &EntityStorage {
		&self.entities
	}

	pub(crate) fn component_storages(&self) -> &ComponentStorages {
		&self.storages
	}

	pub(crate) fn maintain(&mut self) {
		self.entities.maintain();
	}
}
