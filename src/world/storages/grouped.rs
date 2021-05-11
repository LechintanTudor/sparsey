use crate::components::{ComponentStorage, Entity};
use crate::world::{GroupInfoData, GroupMask, Layout};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::mem;

#[derive(Default)]
pub(crate) struct GroupedComponentStorages {
	group_sets: Vec<GroupSet>,
	info: HashMap<TypeId, ComponentInfo>,
}

unsafe impl Send for GroupedComponentStorages {}
unsafe impl Sync for GroupedComponentStorages {}

impl GroupedComponentStorages {
	pub fn with_layout(
		layout: &Layout,
		sparse_set_map: &mut HashMap<TypeId, ComponentStorage>,
	) -> Self {
		let mut group_sets = Vec::<GroupSet>::new();
		let mut info = HashMap::<TypeId, ComponentInfo>::new();

		for group_layout in layout.group_sets() {
			let mut storages = Vec::<AtomicRefCell<ComponentStorage>>::new();
			let mut groups = Vec::<Group>::new();

			let components = group_layout.components();
			let mut prev_arity = 0_usize;

			for (group_index, &arity) in group_layout.arities().iter().enumerate() {
				for component in &components[prev_arity..arity] {
					let type_id = component.type_id();

					info.insert(
						type_id,
						ComponentInfo {
							group_set_index: group_sets.len(),
							storage_index: storages.len(),
							group_index,
						},
					);

					let sparse_set = match sparse_set_map.remove(&type_id) {
						Some(sparse_set) => sparse_set,
						None => component.new_storage().1,
					};

					storages.push(AtomicRefCell::new(sparse_set));
				}

				groups.push(Group::new(arity, prev_arity));
				prev_arity = arity;
			}

			group_sets.push(GroupSet {
				storages: storages,
				groups,
			});
		}

		Self { group_sets, info }
	}

	pub fn clear(&mut self) {
		for group in self.group_sets.iter_mut() {
			for sparse_set in group.storages.iter_mut() {
				sparse_set.get_mut().clear();
			}

			for group in group.groups.iter_mut() {
				group.len = 0;
			}
		}
	}

	pub fn drain_into(&mut self, storages: &mut HashMap<TypeId, ComponentStorage>) {
		for (&type_id, info) in self.info.iter() {
			let storage = self.group_sets[info.group_index].storages[info.storage_index].get_mut();
			let storage = mem::replace(storage, ComponentStorage::for_type::<()>());
			storages.insert(type_id, storage);
		}

		self.info.clear();
		self.group_sets.clear();
	}

	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.info.contains_key(type_id)
	}

	pub fn group_components(&mut self, group_index: usize, entity: Entity) {
		let (storages, groups) = {
			let group = &mut self.group_sets[group_index];
			(group.storages.as_mut_slice(), group.groups.as_mut_slice())
		};

		let mut prev_arity = 0_usize;

		for group in groups.iter_mut() {
			let status =
				get_group_status(&mut storages[prev_arity..group.arity], group.len, entity);

			match status {
				GroupStatus::Grouped => (),
				GroupStatus::Ungrouped => unsafe {
					group_components(&mut storages[..group.arity], &mut group.len, entity);
				},
				GroupStatus::MissingComponents => break,
			}

			prev_arity = group.arity;
		}
	}

	pub fn ungroup_components(&mut self, group_index: usize, entity: Entity) {
		let (storages, groups) = {
			let group = &mut self.group_sets[group_index];
			(group.storages.as_mut_slice(), group.groups.as_mut_slice())
		};

		let mut prev_arity = 0_usize;
		let mut ungroup_start = 0_usize;
		let mut ungroup_len = 0_usize;

		for (i, group) in groups.iter_mut().enumerate() {
			let status =
				get_group_status(&mut storages[prev_arity..group.arity], group.len, entity);

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

			prev_arity = group.arity;
		}

		let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

		for group in (&mut groups[ungroup_range]).iter_mut().rev() {
			unsafe {
				ungroup_components(&mut storages[..group.arity], &mut group.len, entity);
			}
		}
	}

	pub fn group_set_count(&self) -> usize {
		self.group_sets.len()
	}

	pub fn get_group_set_index(&self, type_id: &TypeId) -> Option<usize> {
		self.info.get(type_id).map(|info| info.group_set_index)
	}

	pub fn borrow_with_info(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRef<ComponentStorage>, GroupInfoData)> {
		self.info.get(component).map(|info| unsafe {
			let storage = self
				.group_sets
				.get_unchecked(info.group_set_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow();

			let info = GroupInfoData::new(
				&self.group_sets.get_unchecked(info.group_set_index).groups,
				info.group_index as _,
				info.storage_index as _,
			);

			(storage, info)
		})
	}

	pub fn borrow_with_info_mut(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRefMut<ComponentStorage>, GroupInfoData)> {
		self.info.get(component).map(|info| unsafe {
			let storage = self
				.group_sets
				.get_unchecked(info.group_set_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow_mut();

			let info = GroupInfoData::new(
				&self.group_sets.get_unchecked(info.group_set_index).groups,
				info.group_index as _,
				info.storage_index as _,
			);

			(storage, info)
		})
	}

	pub fn borrow(&self, type_id: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
		self.info.get(type_id).map(|info| unsafe {
			self.group_sets
				.get_unchecked(info.group_set_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow()
		})
	}

	pub fn borrow_mut(&self, type_id: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
		self.info.get(type_id).map(|info| unsafe {
			self.group_sets
				.get_unchecked(info.group_set_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow_mut()
		})
	}

	pub fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
		self.group_sets.iter_mut().flat_map(|group| {
			group
				.storages
				.iter_mut()
				.map(|sparse_set| sparse_set.get_mut())
		})
	}
}

#[derive(Default)]
struct GroupSet {
	storages: Vec<AtomicRefCell<ComponentStorage>>,
	groups: Vec<Group>,
}

#[derive(Copy, Clone)]
struct ComponentInfo {
	group_set_index: usize,
	storage_index: usize,
	group_index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GroupStatus {
	MissingComponents,
	Ungrouped,
	Grouped,
}

fn get_group_status(
	storages: &mut [AtomicRefCell<ComponentStorage>],
	group_len: usize,
	entity: Entity,
) -> GroupStatus {
	match storages.split_first_mut() {
		Some((first, others)) => {
			let status = match first.get_mut().get_index(entity) {
				Some(index) => {
					if (index as usize) < group_len {
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
	storages: &mut [AtomicRefCell<ComponentStorage>],
	group_len: &mut usize,
	entity: Entity,
) {
	for sparse_set in storages.iter_mut().map(|sparse_set| sparse_set.get_mut()) {
		let index = match sparse_set.get_index(entity) {
			Some(index) => index as usize,
			None => unreachable_unchecked(),
		};

		sparse_set.swap(index, *group_len);
	}

	*group_len += 1;
}

unsafe fn ungroup_components(
	storages: &mut [AtomicRefCell<ComponentStorage>],
	group_len: &mut usize,
	entity: Entity,
) {
	if *group_len > 0 {
		let last_index = *group_len - 1;

		for sparse_set in storages.iter_mut().map(|sparse_set| sparse_set.get_mut()) {
			let index = match sparse_set.get_index(entity) {
				Some(index) => index as usize,
				None => unreachable_unchecked(),
			};

			sparse_set.swap(index, last_index);
		}

		*group_len -= 1;
	}
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Group {
	arity: usize,
	include_mask: GroupMask,
	exclude_mask: GroupMask,
	len: usize,
}

impl Group {
	fn new(arity: usize, prev_arity: usize) -> Self {
		Self {
			arity,
			include_mask: GroupMask::new_include_group(arity),
			exclude_mask: GroupMask::new_exclude_group(arity, prev_arity),
			len: 0,
		}
	}

	pub fn include_mask(&self) -> GroupMask {
		self.include_mask
	}

	pub fn exclude_mask(&self) -> GroupMask {
		self.exclude_mask
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
