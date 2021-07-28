use crate::components::{
	Component, ComponentStorage, ComponentTicks, Entity, Ticks, TypedComponentStorage,
};
use crate::world::GroupInfo;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::ops::Deref;

type ComponentStorageRef<'a, T> = TypedComponentStorage<AtomicRef<'a, ComponentStorage>, T>;
type ComponentStorageRefMut<'a, T> = TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>;

/// Shared view over a component storage.
pub struct Comp<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRef<'a, T>,
	pub(crate) group_info: GroupInfo<'a>,
	pub(crate) world_tick: Ticks,
	pub(crate) last_system_tick: Ticks,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRef<'a, ComponentStorage>,
		group_info: GroupInfo<'a>,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Self {
		Self {
			storage: ComponentStorageRef::new(storage),
			group_info,
			world_tick,
			last_system_tick,
		}
	}

	/// Returns `entity`'s component, if `entity` exists.
	pub fn get(&self, entity: Entity) -> Option<&T> {
		self.storage.get(entity)
	}

	/// Returns `entity`'s component ticks, if `entity` exists.
	pub fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	/// Returns `entity`'s component and its ticks, if `entity` exists.
	pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ComponentTicks)> {
		self.storage.get_with_ticks(entity)
	}

	/// Returns `true` if the view contains `entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	/// Returns the number of components in the view.
	pub fn len(&self) -> usize {
		self.storage.len()
	}

	/// Returns `true` if the view is empty.
	pub fn is_empty(&self) -> bool {
		self.storage.is_empty()
	}

	/// Returns all entities in the storage.
	pub fn entities(&self) -> &[Entity] {
		self.storage.entities()
	}

	/// Returns all components in the storage.
	pub fn components(&self) -> &[T] {
		self.storage.components()
	}

	/// Returns all component ticks in the storage.
	pub fn ticks(&self) -> &[ComponentTicks] {
		self.storage.ticks()
	}
}

impl<T> Deref for Comp<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.storage.components()
	}
}

/// Exclusive view over a component storage.
pub struct CompMut<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRefMut<'a, T>,
	pub(crate) group_info: GroupInfo<'a>,
	pub(crate) world_tick: Ticks,
	pub(crate) last_system_tick: Ticks,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRefMut<'a, ComponentStorage>,
		group_info: GroupInfo<'a>,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Self {
		Self {
			storage: ComponentStorageRefMut::new(storage),
			group_info,
			world_tick,
			last_system_tick,
		}
	}

	/// Returns `entity`'s component, if `entity` exists.
	pub fn get(&self, entity: Entity) -> Option<&T> {
		self.storage.get(entity)
	}

	/// Returns `entity`'s component ticks, if `entity` exists.
	pub fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	/// Returns `entity`'s component and its ticks, if `entity` exists.
	pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ComponentTicks)> {
		self.storage.get_with_ticks(entity)
	}

	/// Returns `true` if the view contains `entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	/// Returns the number of components in the view.
	pub fn len(&self) -> usize {
		self.storage.len()
	}

	/// Returns `true` if the view is empty.
	pub fn is_empty(&self) -> bool {
		self.storage.is_empty()
	}

	/// Returns all entities in the storage.
	pub fn entities(&self) -> &[Entity] {
		self.storage.entities()
	}

	/// Returns all components in the storage.
	pub fn components(&self) -> &[T] {
		self.storage.components()
	}

	/// Returns all component ticks in the storage.
	pub fn ticks(&self) -> &[ComponentTicks] {
		self.storage.ticks()
	}
}

impl<T> Deref for CompMut<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.storage.components()
	}
}
