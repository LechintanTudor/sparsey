use crate::components::{Entity, IndexEntity};
use std::hint::unreachable_unchecked;
use std::{iter, ptr};

const PAGE_SIZE: usize = 32;
type EntityPage = Option<Box<[Option<IndexEntity>; PAGE_SIZE]>>;

/// Data structure which maps `Entities` to indexes.
#[derive(Clone, Debug, Default)]
pub struct SparseArray {
	pages: Vec<EntityPage>,
}

impl SparseArray {
	/// Remove the `IndexEntity` at the given `Entity`.
	pub fn remove(&mut self, entity: Entity) -> Option<IndexEntity> {
		self.get_mut(entity).map(|e| e.take()).flatten()
	}

	/// Delete the `IndexEntity` at the given `Entity`.
	pub fn delete(&mut self, entity: Entity) {
		if let Some(entity) = self.get_mut(entity) {
			*entity = None;
		}
	}

	/// Remove all `Entities` in the `SparseArray`.
	pub fn clear(&mut self) {
		self.pages.iter_mut().for_each(|p| *p = None);
	}

	/// Check if the `SparseArray` contains the given `Entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.get_index(entity).is_some()
	}

	/// Get the `IndexEntity` at the given `Entity`.
	pub fn get_index(&self, entity: Entity) -> Option<usize> {
		self.pages
			.get(page_index(entity))
			.and_then(|p| p.as_ref())
			.and_then(|p| p[local_index(entity)])
			.filter(|e| e.version() == entity.version())
			.map(|e| e.index())
	}

	/// Get an exclusive reference to the `IndexEntity` slot at the given
	/// `Entity`.
	pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Option<IndexEntity>> {
		self.pages
			.get_mut(page_index(entity))
			.and_then(|page| page.as_mut())
			.map(|page| &mut page[local_index(entity)])
			.filter(|e| e.map(|e| e.version()) == Some(entity.version()))
	}

	/// Get an excusive reference to the `IndexEntity` slot at the given `index`
	/// without checking the validity of the `index`.
	pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
		match self.pages.get_unchecked_mut(index / PAGE_SIZE) {
			Some(page) => page.get_unchecked_mut(index % PAGE_SIZE),
			None => unreachable_unchecked(),
		}
	}

	/// Get an exclusive reference to the `IndexEntity` slot at the given
	/// `index`. If the `index` is larger than the maximum capacity of the
	/// `SparseArray`, extra memory is allocated to accomodate the new slot.
	pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<IndexEntity> {
		let page_index = index / PAGE_SIZE;

		if page_index < self.pages.len() {
			let page = &mut self.pages[page_index];

			if page.is_none() {
				*page = empty_page();
			}
		} else {
			let extra_uninit_pages = page_index - self.pages.len();
			self.pages.reserve(extra_uninit_pages + 1);
			self.pages
				.extend(iter::repeat(uninit_page()).take(extra_uninit_pages));
			self.pages.push(empty_page())
		}

		unsafe {
			match self.pages.get_unchecked_mut(page_index) {
				Some(page) => page.get_unchecked_mut(index % PAGE_SIZE),
				None => unreachable_unchecked(),
			}
		}
	}

	/// Swap the `IndexEntities` at the given indexes without checking if
	/// the indexes are valid and in bounds.
	pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
		let pa = self.get_unchecked_mut(a) as *mut _;
		let pb = self.get_unchecked_mut(b) as *mut _;
		ptr::swap(pa, pb);
	}

	pub fn as_view(&self) -> SparseArrayView {
		SparseArrayView { pages: &self.pages }
	}
}

#[derive(Copy, Clone)]
pub struct SparseArrayView<'a> {
	pages: &'a [EntityPage],
}

impl SparseArrayView<'_> {
	/// Check if the `SparseArray` contains the given `Entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.get_index(entity).is_some()
	}

	/// Get the `IndexEntity` at the given `Entity`.
	pub fn get_index(&self, entity: Entity) -> Option<usize> {
		self.pages
			.get(page_index(entity))
			.and_then(|p| p.as_ref())
			.and_then(|p| p[local_index(entity)])
			.filter(|e| e.version() == entity.version())
			.map(|e| e.index())
	}
}

fn page_index(entity: Entity) -> usize {
	entity.index() as usize / PAGE_SIZE
}

fn local_index(entity: Entity) -> usize {
	entity.index() as usize % PAGE_SIZE
}

fn uninit_page() -> EntityPage {
	None
}

fn empty_page() -> EntityPage {
	Some(Box::new([None; PAGE_SIZE]))
}
