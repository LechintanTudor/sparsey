use std::hint::unreachable_unchecked;
use std::{iter, mem};

const PAGE_SIZE: usize = 32;
type Page = Option<Box<[usize; PAGE_SIZE]>>;

#[derive(Default, Debug)]
pub struct SparseArray {
	pages: Vec<Page>,
}

impl SparseArray {
	pub const INVALID_INDEX: usize = usize::MAX;

	pub const fn new() -> Self {
		Self { pages: Vec::new() }
	}

	pub fn insert(&mut self, index: usize, value: usize) -> usize {
		mem::replace(self.get_mut_or_invalid(index), value)
	}

	pub fn remove(&mut self, index: usize) -> usize {
		match self.get_mut(index) {
			Some(value) => mem::replace(value, Self::INVALID_INDEX),
			None => Self::INVALID_INDEX,
		}
	}

	pub fn contains(&self, index: usize) -> bool {
		self.get(index).is_some()
	}

	pub fn get(&self, index: usize) -> Option<&usize> {
		let (page_index, local_index) = indexes(index);

		self.pages
			.get(page_index)
			.and_then(|page| page.as_ref())
			.map(|page| &page[local_index])
			.filter(|index| **index != Self::INVALID_INDEX)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut usize> {
		let (page_index, local_index) = indexes(index);

		self.pages
			.get_mut(page_index)
			.and_then(|page| page.as_mut())
			.map(|page| &mut page[local_index])
			.filter(|index| **index != Self::INVALID_INDEX)
	}

	pub fn get_mut_or_invalid(&mut self, index: usize) -> &mut usize {
		let (page_index, local_index) = indexes(index);

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
			self.pages.push(empty_page());
		}

		unsafe {
			match self.pages.get_unchecked_mut(page_index) {
				Some(page) => page.get_unchecked_mut(local_index),
				None => unreachable_unchecked(),
			}
		}
	}

	pub unsafe fn get_unchecked(&self, index: usize) -> &usize {
		let (page_index, local_index) = indexes(index);

		match self.pages.get_unchecked(page_index) {
			Some(page) => page.get_unchecked(local_index),
			None => unreachable_unchecked(),
		}
	}

	pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut usize {
		let (page_index, local_index) = indexes(index);

		match self.pages.get_unchecked_mut(page_index) {
			Some(page) => page.get_unchecked_mut(local_index),
			None => unreachable_unchecked(),
		}
	}

	pub fn clear(&mut self) {
		self.pages.clear();
	}
}

fn indexes(index: usize) -> (usize, usize) {
	(index / PAGE_SIZE, index % PAGE_SIZE)
}

fn uninit_page() -> Page {
	None
}

fn empty_page() -> Page {
	Some(Box::new([SparseArray::INVALID_INDEX; PAGE_SIZE]))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sparse_array() {
		let mut array = SparseArray::default();

		assert_eq!(array.insert(0, 1), SparseArray::INVALID_INDEX);
		assert_eq!(array.insert(1, 2), SparseArray::INVALID_INDEX);
		assert_eq!(array.insert(0, 0), 1);
		assert_eq!(array.insert(1, 1), 2);

		assert!(matches!(array.get(0), Some(&0)));
		assert!(matches!(array.get(1), Some(&1)));
		assert!(matches!(array.get(2), None));
		assert!(matches!(array.get(3), None));

		assert!(matches!(array.get_mut(0), Some(&mut 0)));
		assert!(matches!(array.get_mut(1), Some(&mut 1)));
		assert!(matches!(array.get_mut(2), None));
		assert!(matches!(array.get_mut(3), None));

		*array.get_mut(0).unwrap() = 1;
		*array.get_mut(1).unwrap() = 2;

		assert_eq!(*array.get_mut_or_invalid(0), 1);
		assert_eq!(*array.get_mut_or_invalid(1), 2);
		assert_eq!(*array.get_mut_or_invalid(100), SparseArray::INVALID_INDEX);
		assert_eq!(*array.get_mut_or_invalid(200), SparseArray::INVALID_INDEX);

		assert_eq!(array.remove(0), 1);
		assert_eq!(array.remove(1), 2);
		assert_eq!(array.remove(0), SparseArray::INVALID_INDEX);
		assert_eq!(array.remove(1), SparseArray::INVALID_INDEX);
	}
}
