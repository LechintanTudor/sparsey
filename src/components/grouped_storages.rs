use crate::group::{Group, GroupInfo};
use crate::layout::Layout;
use crate::storage::{ComponentStorage, Entity};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::hint::unreachable_unchecked;
use std::mem;

#[derive(Default)]
pub(crate) struct GroupedComponentStorages {
	families: Vec<GroupFamily>,
	info: FxHashMap<TypeId, ComponentInfo>,
}

unsafe impl Send for GroupedComponentStorages {}
unsafe impl Sync for GroupedComponentStorages {}

impl GroupedComponentStorages {
	pub fn with_layout(
		layout: &Layout,
		storage_map: &mut FxHashMap<TypeId, ComponentStorage>,
	) -> Self {
		let mut families = Vec::<GroupFamily>::new();
		let mut info = FxHashMap::<TypeId, ComponentInfo>::default();

		for group_layout in layout.group_families() {
			let mut storages = Vec::<AtomicRefCell<ComponentStorage>>::new();
			let mut groups = Vec::<Group>::new();

			let components = group_layout.components();
			let mut prev_arity = 0_usize;

			for (group_index, &arity) in group_layout.group_arities().iter().enumerate() {
				for component in &components[prev_arity..arity] {
					let type_id = component.type_id();

					info.insert(
						type_id,
						ComponentInfo {
							group_family_index: families.len(),
							storage_index: storages.len(),
							group_index,
						},
					);

					let storage = match storage_map.remove(&type_id) {
						Some(storage) => storage,
						None => component.new_storage().1,
					};

					storages.push(AtomicRefCell::new(storage));
				}

				groups.push(Group::new(prev_arity, arity));
				prev_arity = arity;
			}

			families.push(GroupFamily { storages, groups });
		}

		Self { families, info }
	}

	pub fn drain_into(&mut self, storages: &mut FxHashMap<TypeId, ComponentStorage>) {
		for (&type_id, info) in self.info.iter() {
			let storage = self.families[info.group_index].storages[info.storage_index].get_mut();
			let storage = mem::replace(storage, ComponentStorage::for_type::<()>());
			storages.insert(type_id, storage);
		}

		self.info.clear();
		self.families.clear();
	}

	pub unsafe fn group_components(&mut self, group_index: usize, entity: Entity) {
		let (storages, groups) = {
			let group = self.families.get_unchecked_mut(group_index);
			(group.storages.as_mut_slice(), group.groups.as_mut_slice())
		};

		let mut prev_arity = 0_usize;

		for group in groups.iter_mut() {
			let status = get_group_status(
				storages.get_unchecked_mut(prev_arity..group.arity()),
				group.len,
				entity,
			);

			match status {
				GroupStatus::Grouped => (),
				GroupStatus::Ungrouped => {
					group_components(
						storages.get_unchecked_mut(..group.arity()),
						&mut group.len,
						entity,
					);
				}
				GroupStatus::MissingComponents => break,
			}

			prev_arity = group.arity();
		}
	}

	pub unsafe fn ungroup_components(&mut self, group_index: usize, entity: Entity) {
		let (storages, groups) = {
			let group = self.families.get_unchecked_mut(group_index);
			(group.storages.as_mut_slice(), group.groups.as_mut_slice())
		};

		let mut prev_arity = 0_usize;
		let mut ungroup_start = 0_usize;
		let mut ungroup_len = 0_usize;

		for (i, group) in groups.iter_mut().enumerate() {
			let status = get_group_status(
				storages.get_unchecked_mut(prev_arity..group.arity()),
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

			prev_arity = group.arity();
		}

		let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

		for group in groups.get_unchecked_mut(ungroup_range).iter_mut().rev() {
			ungroup_components(
				storages.get_unchecked_mut(..group.arity()),
				&mut group.len,
				entity,
			);
		}
	}

	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.info.contains_key(type_id)
	}

	pub fn group_family_count(&self) -> usize {
		self.families.len()
	}

	pub fn group_family_of(&self, component: &TypeId) -> Option<usize> {
		self.info.get(component).map(|info| info.group_family_index)
	}

	pub fn borrow_with_info(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRef<ComponentStorage>, GroupInfo)> {
		self.info.get(component).map(|info| unsafe {
			let storage = self
				.families
				.get_unchecked(info.group_family_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow();

			let info = GroupInfo::new(
				&self.families.get_unchecked(info.group_family_index).groups,
				info.group_index,
				info.storage_index,
			);

			(storage, info)
		})
	}

	pub fn borrow_with_info_mut(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRefMut<ComponentStorage>, GroupInfo)> {
		self.info.get(component).map(|info| unsafe {
			let storage = self
				.families
				.get_unchecked(info.group_family_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow_mut();

			let info = GroupInfo::new(
				&self.families.get_unchecked(info.group_family_index).groups,
				info.group_index,
				info.storage_index,
			);

			(storage, info)
		})
	}

	pub fn borrow_with_familiy_mut(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRefMut<ComponentStorage>, usize)> {
		self.info.get(component).map(|info| unsafe {
			let storage = self
				.families
				.get_unchecked(info.group_family_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow_mut();

			(storage, info.group_family_index)
		})
	}

	pub fn borrow(&self, type_id: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
		self.info.get(type_id).map(|info| unsafe {
			self.families
				.get_unchecked(info.group_family_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow()
		})
	}

	pub fn borrow_mut(&self, type_id: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
		self.info.get(type_id).map(|info| unsafe {
			self.families
				.get_unchecked(info.group_family_index)
				.storages
				.get_unchecked(info.storage_index)
				.borrow_mut()
		})
	}

	pub fn clear(&mut self) {
		for group in self.families.iter_mut() {
			for storage in group.storages.iter_mut() {
				storage.get_mut().clear();
			}

			for group in group.groups.iter_mut() {
				group.len = 0;
			}
		}
	}

	pub fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
		self.families
			.iter_mut()
			.flat_map(|group| group.storages.iter_mut().map(|storage| storage.get_mut()))
	}
}

#[derive(Default)]
struct GroupFamily {
	storages: Vec<AtomicRefCell<ComponentStorage>>,
	groups: Vec<Group>,
}

#[derive(Copy, Clone)]
struct ComponentInfo {
	group_family_index: usize,
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
				.all(|storage| storage.get_mut().contains(entity))
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
	for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
		let index = match storage.get_index(entity) {
			Some(index) => index as usize,
			None => unreachable_unchecked(),
		};

		storage.swap(index, *group_len);
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

		for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
			let index = match storage.get_index(entity) {
				Some(index) => index as usize,
				None => unreachable_unchecked(),
			};

			storage.swap(index, last_index);
		}

		*group_len -= 1;
	}
}
