use crate::components::layout::repeat_layout;
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr;
use std::ptr::NonNull;

pub struct BlobVec {
	item_layout: Layout,
	drop_item: unsafe fn(*mut u8),
	swap_space: NonNull<u8>,
	ptr: NonNull<u8>,
	cap: usize,
	len: usize,
}

impl BlobVec {
	pub unsafe fn new(item_layout: Layout, drop_item: unsafe fn(*mut u8)) -> Self {
		let ptr = NonNull::new_unchecked(item_layout.align() as *mut u8);

		if item_layout.size() == 0 {
			// Times 2 so we can use ptr::copy_nonoverlapping on ZST
			let swap_space = NonNull::new_unchecked((item_layout.align() * 2) as *mut u8);

			Self {
				item_layout,
				drop_item,
				swap_space,
				ptr,
				cap: usize::MAX,
				len: 0,
			}
		} else {
			let swap_space =
				NonNull::new(alloc(item_layout)).unwrap_or_else(|| handle_alloc_error(item_layout));

			Self {
				item_layout,
				drop_item,
				swap_space,
				ptr,
				cap: 0,
				len: 0,
			}
		}
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn capacity(&self) -> usize {
		self.cap
	}

	pub fn clear(&mut self) {
		let len = self.len;
		self.len = 0;

		for i in 0..len {
			unsafe {
				(self.drop_item)(self.ptr.as_ptr().add(i * self.item_layout.size()));
			}
		}
	}

	pub unsafe fn push(&mut self, item: *const u8) {
		if self.len == self.cap {
			self.grow();
		}

		ptr::copy(item, self.get_unchecked(self.len), self.item_layout.size());
		self.len += 1;
	}

	pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
		// Copy first element to swap space
		ptr::copy_nonoverlapping(
			self.get_unchecked(a),
			self.swap_space.as_ptr(),
			self.item_layout.size(),
		);

		// Overwrite first element with second element
		ptr::copy(
			self.get_unchecked(b),
			self.get_unchecked(a),
			self.item_layout.size(),
		);

		// Overwrite second element with first element
		ptr::copy_nonoverlapping(
			self.swap_space.as_ptr(),
			self.get_unchecked(b),
			self.item_layout.size(),
		);
	}

	pub unsafe fn swap_remove_and_forget_unchecked(&mut self, index: usize) -> *mut u8 {
		let last_index = self.len - 1;

		// Copy current element to swap space
		ptr::copy_nonoverlapping(
			self.get_unchecked(index),
			self.swap_space.as_ptr(),
			self.item_layout.size(),
		);

		// Overwrite current element with last element
		ptr::copy(
			self.get_unchecked(last_index),
			self.get_unchecked(index),
			self.item_layout.size(),
		);

		self.len = last_index;
		self.swap_space.as_ptr()
	}

	pub unsafe fn swap_remove_and_drop_unchecked(&mut self, index: usize) {
		(self.drop_item)(self.get_unchecked(index));

		let last_index = self.len - 1;

		// Overwrite current element with last element
		ptr::copy(
			self.get_unchecked(last_index),
			self.get_unchecked(index),
			self.item_layout.size(),
		);

		self.len = last_index;
	}

	pub unsafe fn get_unchecked(&self, index: usize) -> *mut u8 {
		self.ptr.as_ptr().add(index * self.item_layout.size())
	}

	pub unsafe fn set_and_forget_prev_unchecked(
		&mut self,
		index: usize,
		value: *const u8,
	) -> *mut u8 {
		ptr::copy_nonoverlapping(
			self.get_unchecked(index),
			self.swap_space.as_ptr(),
			self.item_layout.size(),
		);
		ptr::copy(value, self.get_unchecked(index), self.item_layout.size());
		self.swap_space.as_ptr()
	}

	pub unsafe fn set_and_drop_prev_unchecked(&mut self, index: usize, value: *const u8) {
		(self.drop_item)(self.get_unchecked(index));
		ptr::copy(value, self.get_unchecked(index), self.item_layout.size());
	}

	pub fn as_ptr(&self) -> *mut u8 {
		self.ptr.as_ptr()
	}

	fn grow(&mut self) {
		assert!(self.item_layout.size() != 0, "BlobVec is overfull");

		unsafe {
			let (new_ptr, new_layout, new_cap) = if self.cap == 0 {
				(alloc(self.item_layout), self.item_layout, 1)
			} else {
				let new_cap = 2 * self.cap;
				let new_layout =
					repeat_layout(&self.item_layout, new_cap).expect("Layout should be valid");

				(
					realloc(self.ptr.as_ptr(), new_layout, new_layout.size()),
					new_layout,
					new_cap,
				)
			};

			if new_ptr.is_null() {
				handle_alloc_error(new_layout);
			}

			self.ptr = NonNull::new_unchecked(new_ptr);
			self.cap = new_cap;
		}
	}
}

impl Drop for BlobVec {
	fn drop(&mut self) {
		self.clear();

		if self.item_layout.size() != 0 {
			unsafe {
				dealloc(self.swap_space.as_ptr(), self.item_layout);
			}

			if self.cap != 0 {
				unsafe {
					dealloc(
						self.ptr.as_ptr(),
						repeat_layout(&self.item_layout, self.cap).expect("Layout should be valid"),
					);
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::{mem, ptr};

	fn new() -> BlobVec {
		unsafe {
			BlobVec::new(Layout::new::<i32>(), |ptr| {
				ptr::drop_in_place::<i32>(ptr as _)
			})
		}
	}

	unsafe fn push(blob_vec: &mut BlobVec, item: i32) {
		blob_vec.push(&item as *const i32 as *const _);
		mem::forget(item);
	}

	unsafe fn get(blob_vec: &BlobVec, index: usize) -> i32 {
		ptr::read(blob_vec.get_unchecked(index) as *mut i32)
	}

	unsafe fn swap_remove(blob_vec: &mut BlobVec, index: usize) -> i32 {
		ptr::read(blob_vec.swap_remove_and_forget_unchecked(index) as *mut i32)
	}

	#[test]
	fn blob_vec() {
		let mut blob_vec = new();

		unsafe {
			// Push
			push(&mut blob_vec, 0);
			assert_eq!(blob_vec.len(), 1);
			assert_eq!(get(&blob_vec, 0), 0);

			push(&mut blob_vec, 1);
			assert_eq!(blob_vec.len(), 2);
			assert_eq!(get(&blob_vec, 0), 0);
			assert_eq!(get(&blob_vec, 1), 1);

			push(&mut blob_vec, 2);
			assert_eq!(blob_vec.len(), 3);
			assert_eq!(get(&blob_vec, 0), 0);
			assert_eq!(get(&blob_vec, 1), 1);
			assert_eq!(get(&blob_vec, 2), 2);

			// Swap
			blob_vec.swap_unchecked(0, 2);
			assert_eq!(get(&blob_vec, 0), 2);
			assert_eq!(get(&blob_vec, 1), 1);
			assert_eq!(get(&blob_vec, 2), 0);

			blob_vec.swap_unchecked(1, 1);
			assert_eq!(get(&blob_vec, 0), 2);
			assert_eq!(get(&blob_vec, 1), 1);
			assert_eq!(get(&blob_vec, 2), 0);

			// Swap remove
			assert_eq!(swap_remove(&mut blob_vec, 0), 2);
			assert_eq!(blob_vec.len(), 2);
			assert_eq!(get(&blob_vec, 0), 0);
			assert_eq!(get(&blob_vec, 1), 1);

			assert_eq!(swap_remove(&mut blob_vec, 1), 1);
			assert_eq!(blob_vec.len(), 1);
			assert_eq!(get(&blob_vec, 0), 0);
		}
	}
}
