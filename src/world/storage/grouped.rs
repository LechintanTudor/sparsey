use crate::data::{AtomicRef, AtomicRefCell, AtomicRefMut, Entity, TypeErasedSparseSet};
use crate::world::{Group, GroupInfo, Layout};
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

#[derive(Default)]
pub(crate) struct GroupedComponents {
	group_sets: Vec<GroupSet>,
	info: HashMap<TypeId, ComponentInfo>,
}

impl GroupedComponents {
	pub fn with_layout(
		layout: &Layout,
		sparse_set_map: &mut HashMap<TypeId, TypeErasedSparseSet>,
	) -> Self {
		let mut group_sets = Vec::<GroupSet>::new();
		let mut info = HashMap::<TypeId, ComponentInfo>::new();

		for group_layout in layout.group_sets() {
			let mut sparse_sets = Vec::<AtomicRefCell<TypeErasedSparseSet>>::new();
			let mut groups = Vec::<Group>::new();

			let components = group_layout.components();
			let mut previous_arity = 0_usize;

			for (group_index, &arity) in group_layout.arities().iter().enumerate() {
				for component in &components[previous_arity..arity] {
					let type_id = component.type_id();

					info.insert(
						type_id,
						ComponentInfo {
							group_set_index: group_sets.len(),
							sparse_set_index: sparse_sets.len(),
							group_index,
						},
					);

					let sparse_set = match sparse_set_map.remove(&type_id) {
						Some(sparse_set) => sparse_set,
						None => component.new_sparse_set(),
					};

					sparse_sets.push(AtomicRefCell::new(sparse_set));
				}

				groups.push(Group::with_arity(arity));
				previous_arity = arity;
			}

			group_sets.push(GroupSet {
				sparse_sets,
				groups,
			});
		}

		Self { group_sets, info }
	}

	pub fn clear(&mut self) {
		for group in self.group_sets.iter_mut() {
			for sparse_set in group.sparse_sets.iter_mut() {
				sparse_set.get_mut().clear();
			}

			for group in group.groups.iter_mut() {
				group.len = 0;
			}
		}
	}

	pub fn drain(&mut self) -> impl Iterator<Item = TypeErasedSparseSet> + '_ {
		self.info.clear();

		self.group_sets
			.iter_mut()
			.flat_map(|group| group.sparse_sets.drain(..))
			.map(|sparse_set| sparse_set.into_inner())
	}

	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.info.contains_key(type_id)
	}

	pub fn group_components(&mut self, group_index: usize, entity: Entity) {
		let (sparse_sets, groups) = {
			let group = &mut self.group_sets[group_index];
			(
				group.sparse_sets.as_mut_slice(),
				group.groups.as_mut_slice(),
			)
		};

		let mut previous_arity = 0_usize;

		for group in groups.iter_mut() {
			let status = get_group_status(
				&mut sparse_sets[previous_arity..group.arity()],
				group.len,
				entity,
			);

			match status {
				GroupStatus::Grouped => (),
				GroupStatus::Ungrouped => unsafe {
					group_components(&mut sparse_sets[..group.arity()], &mut group.len, entity);
				},
				GroupStatus::MissingComponents => break,
			}

			previous_arity = group.arity();
		}
	}

	pub fn ungroup_components(&mut self, group_index: usize, entity: Entity) {
		let (sparse_sets, groups) = {
			let group = &mut self.group_sets[group_index];
			(
				group.sparse_sets.as_mut_slice(),
				group.groups.as_mut_slice(),
			)
		};

		let mut previous_arity = 0_usize;
		let mut ungroup_start = 0_usize;
		let mut ungroup_len = 0_usize;

		for (i, group) in groups.iter_mut().enumerate() {
			let status = get_group_status(
				&mut sparse_sets[previous_arity..group.arity()],
				group.len,
				entity,
			);

			match status {
				GroupStatus::Grouped => {
					if ungroup_len == 0 {
						ungroup_start = i;
					}

					ungroup_len += 1;
				}
				GroupStatus::Ungrouped => break,
				GroupStatus::MissingComponents => break,
			}

			previous_arity = group.arity();
		}

		let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

		for group in (&mut groups[ungroup_range]).iter_mut().rev() {
			unsafe {
				ungroup_components(&mut sparse_sets[..group.arity()], &mut group.len, entity);
			}
		}
	}

	pub fn group_set_count(&self) -> usize {
		self.group_sets.len()
	}

	pub fn get_group_set_index(&self, type_id: &TypeId) -> Option<usize> {
		self.info.get(type_id).map(|info| info.group_set_index)
	}

	pub fn get_group_info(&self, type_id: &TypeId) -> Option<GroupInfo> {
		self.info.get(type_id).map(|info| unsafe {
			let groups = &self.group_sets.get_unchecked(info.group_set_index).groups;
			let group_index = info.group_index;
			let sparse_set_index = info.sparse_set_index;

			GroupInfo::new(groups, group_index, sparse_set_index)
		})
	}

	pub fn borrow(&self, type_id: &TypeId) -> Option<AtomicRef<TypeErasedSparseSet>> {
		self.info.get(type_id).map(|info| unsafe {
			self.group_sets
				.get_unchecked(info.group_set_index)
				.sparse_sets
				.get_unchecked(info.sparse_set_index)
				.borrow()
		})
	}

	pub fn borrow_mut(&self, type_id: &TypeId) -> Option<AtomicRefMut<TypeErasedSparseSet>> {
		self.info.get(type_id).map(|info| unsafe {
			self.group_sets
				.get_unchecked(info.group_set_index)
				.sparse_sets
				.get_unchecked(info.sparse_set_index)
				.borrow_mut()
		})
	}

	pub fn iter_sparse_sets_mut(&mut self) -> impl Iterator<Item = &mut TypeErasedSparseSet> {
		self.group_sets.iter_mut().flat_map(|group| {
			group
				.sparse_sets
				.iter_mut()
				.map(|sparse_set| sparse_set.get_mut())
		})
	}
}

#[derive(Default)]
struct GroupSet {
	sparse_sets: Vec<AtomicRefCell<TypeErasedSparseSet>>,
	groups: Vec<Group>,
}

#[derive(Copy, Clone)]
struct ComponentInfo {
	group_set_index: usize,
	sparse_set_index: usize,
	group_index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GroupStatus {
	MissingComponents,
	Ungrouped,
	Grouped,
}

fn get_group_status(
	sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
	group_len: usize,
	entity: Entity,
) -> GroupStatus {
	match sparse_sets.split_first_mut() {
		Some((first, others)) => {
			let status = match first.get_mut().get_index_entity(entity) {
				Some(index_entity) => {
					if index_entity.index() < group_len {
						GroupStatus::Grouped
					} else {
						GroupStatus::Ungrouped
					}
				}
				None => return GroupStatus::MissingComponents,
			};

			if others
				.iter_mut()
				.all(|sparse_set| sparse_set.get_mut().contains(entity))
			{
				status
			} else {
				GroupStatus::MissingComponents
			}
		}
		None => GroupStatus::Grouped,
	}
}

unsafe fn group_components(
	sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
	group_len: &mut usize,
	entity: Entity,
) {
	for sparse_set in sparse_sets
		.iter_mut()
		.map(|sparse_set| sparse_set.get_mut())
	{
		let index = match sparse_set.get_index_entity(entity) {
			Some(index_entity) => index_entity.index(),
			None => unreachable_unchecked(),
		};

		sparse_set.swap(index, *group_len);
	}

	*group_len += 1;
}

unsafe fn ungroup_components(
	sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
	group_len: &mut usize,
	entity: Entity,
) {
	if *group_len > 0 {
		let last_index = *group_len - 1;

		for sparse_set in sparse_sets
			.iter_mut()
			.map(|sparse_set| sparse_set.get_mut())
		{
			let index = match sparse_set.get_index_entity(entity) {
				Some(index_entity) => index_entity.index(),
				None => unreachable_unchecked(),
			};

			sparse_set.swap(index, last_index);
		}

		*group_len -= 1;
	}
}
