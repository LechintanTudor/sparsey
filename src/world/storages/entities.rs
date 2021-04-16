use crate::components::{Entity, SparseArray};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Default)]
pub(crate) struct EntityStorage {
	storage: EntitySparseSet,
	allocator: EntityAllocator,
}

impl EntityStorage {
	pub fn create(&mut self) -> Entity {
		self.maintain();

		let entity = self
			.allocator
			.allocate()
			.expect("No entities left to allocate");

		self.storage.insert(entity);
		entity
	}

	pub fn create_atomic(&self) -> Entity {
		self.allocator
			.allocate_atomic()
			.expect("No entities left to allocate")
	}

	pub fn destroy(&mut self, entity: Entity) -> bool {
		self.maintain();

		if self.storage.remove(entity) {
			self.allocator.deallocate(entity);
			true
		} else {
			false
		}
	}

	pub fn clear(&mut self) {
		self.storage.clear();
		self.allocator.clear();
	}

	pub fn maintain(&mut self) {
		for entity in self.allocator.maintain() {
			self.storage.insert(entity);
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}
}

impl AsRef<[Entity]> for EntityStorage {
	fn as_ref(&self) -> &[Entity] {
		&self.storage.entities
	}
}

#[derive(Clone, Default, Debug)]
struct EntitySparseSet {
	indexes: SparseArray,
	entities: Vec<Entity>,
}

impl EntitySparseSet {
	fn insert(&mut self, entity: Entity) {
		match self.indexes.get_mut_or_invalid(entity.index()) {
			// New Entity
			index @ &mut SparseArray::INVALID_INDEX => {
				*index = self.entities.len();
				self.entities.push(entity);
			}
			// Newer version of an Entity
			index => unsafe {
				*self.entities.get_unchecked_mut(*index) = entity;
			},
		}
	}

	fn remove(&mut self, entity: Entity) -> bool {
		let dense_index_ref = match self.indexes.get_mut(entity.index()) {
			Some(index) => unsafe {
				if self.entities.get_unchecked(*index).version() == entity.version() {
					index
				} else {
					return false;
				}
			},
			None => return false,
		};

		let sparse_index = match self.entities.last() {
			Some(entity) => entity.index(),
			None => return false,
		};

		let dense_index = *dense_index_ref;
		self.entities.swap_remove(dense_index);

		unsafe {
			*dense_index_ref = SparseArray::INVALID_INDEX;
			*self.indexes.get_unchecked_mut(sparse_index) = dense_index;
		}

		true
	}

	fn contains(&self, entity: Entity) -> bool {
		let index = match self.indexes.get(entity.index()) {
			Some(index) => *index,
			None => return false,
		};

		unsafe { self.entities.get_unchecked(index).version() == entity.version() }
	}

	fn clear(&mut self) {
		self.indexes.clear();
		self.entities.clear();
	}
}

#[derive(Default, Debug)]
struct EntityAllocator {
	current_id: AtomicU32,
	last_id: u32,
	recycled: Vec<Entity>,
	recycled_len: AtomicUsize,
}

impl EntityAllocator {
	fn allocate(&mut self) -> Option<Entity> {
		match self.recycled.pop() {
			Some(entity) => {
				*self.recycled_len.get_mut() -= 1;
				Some(entity)
			}
			None => {
				let current_id = *self.current_id.get_mut();
				*self.current_id.get_mut() = self.current_id.get_mut().checked_add(1)?;
				Some(Entity::with_index(current_id))
			}
		}
	}

	fn allocate_atomic(&self) -> Option<Entity> {
		match atomic_decrement_usize(&self.recycled_len) {
			Some(recycled_len) => Some(self.recycled[recycled_len - 1]),
			None => atomic_increment_u32(&self.current_id).map(|id| Entity::with_index(id)),
		}
	}

	fn deallocate(&mut self, entity: Entity) {
		if let Some(next_entity) = entity.with_next_version() {
			self.recycled.push(next_entity);
			*self.recycled_len.get_mut() += 1;
		}
	}

	fn clear(&mut self) {
		*self.current_id.get_mut() = 0;
		self.last_id = 0;
		self.recycled.clear();
		*self.recycled_len.get_mut() = 0;
	}

	fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
		let remaining = *self.recycled_len.get_mut();
		*self.recycled_len.get_mut() = self.recycled.len();

		let new_id_range = self.last_id..*self.current_id.get_mut();
		self.last_id = *self.current_id.get_mut();

		self.recycled
			.drain(remaining..)
			.chain(new_id_range.into_iter().map(|id| Entity::with_index(id)))
	}
}

/// Like `fetch_sub`, but returns `None` on underflow instead of wrapping.
fn atomic_decrement_usize(value: &AtomicUsize) -> Option<usize> {
	let mut prev = value.load(Ordering::Relaxed);

	while prev != 0 {
		match value.compare_exchange_weak(prev, prev - 1, Ordering::Relaxed, Ordering::Relaxed) {
			Ok(prev) => return Some(prev),
			Err(next_prev) => prev = next_prev,
		}
	}

	None
}

/// Like `fetch_add`, but returns `None` on overflow instead of wrapping.
fn atomic_increment_u32(value: &AtomicU32) -> Option<u32> {
	let mut prev = value.load(Ordering::Relaxed);

	while prev != u32::MAX {
		match value.compare_exchange_weak(prev, prev + 1, Ordering::Relaxed, Ordering::Relaxed) {
			Ok(prev) => return Some(prev),
			Err(next_prev) => prev = next_prev,
		}
	}

	None
}
