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
	pub fn new(item_layout: Layout, drop_item: unsafe fn(*mut u8)) -> Self {
		let ptr = unsafe { NonNull::new_unchecked(item_layout.align() as *mut u8) };

		if item_layout.size() == 0 {
			let swap_space = unsafe { NonNull::new_unchecked(item_layout.align() as *mut u8) };

			Self {
				item_layout,
				drop_item,
				swap_space,
				ptr,
				cap: usize::MAX,
				len: 0,
			}
		} else {
			let swap_space = unsafe {
				NonNull::new(alloc(item_layout)).unwrap_or_else(|| handle_alloc_error(item_layout))
			};

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

		ptr::copy_nonoverlapping(item, self.get_unchecked(self.len), self.item_layout.size());
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
		debug_assert!(index < self.len, "Index out of range");
		self.ptr.as_ptr().add(index * self.item_layout.size())
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

// From https://doc.rust-lang.org/src/core/alloc/layout.rs.html
fn padding_needed_for(layout: &Layout, align: usize) -> usize {
	let len = layout.size();

	// Rounded up value is:
	//   len_rounded_up = (len + align - 1) & !(align - 1);
	// and then we return the padding difference: `len_rounded_up - len`.
	//
	// We use modular arithmetic throughout:
	//
	// 1. align is guaranteed to be > 0, so align - 1 is always
	//    valid.
	//
	// 2. `len + align - 1` can overflow by at most `align - 1`,
	//    so the &-mask with `!(align - 1)` will ensure that in the
	//    case of overflow, `len_rounded_up` will itself be 0.
	//    Thus the returned padding, when added to `len`, yields 0,
	//    which trivially satisfies the alignment `align`.
	//
	// (Of course, attempts to allocate blocks of memory whose
	// size and padding overflow in the above manner should cause
	// the allocator to yield an error anyway.)

	let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
	len_rounded_up.wrapping_sub(len)
}

// From https://doc.rust-lang.org/src/core/alloc/layout.rs.html
fn repeat_layout(layout: &Layout, n: usize) -> Option<Layout> {
	// This cannot overflow. Quoting from the invariant of Layout:
	// > `size`, when rounded up to the nearest multiple of `align`,
	// > must not overflow (i.e., the rounded value must be less than
	// > `usize::MAX`)
	let padded_size = layout.size() + padding_needed_for(layout, layout.align());
	let alloc_size = padded_size.checked_mul(n)?;

	// SAFETY: self.align is already known to be valid and alloc_size has been
	// padded already.
	unsafe {
		Some(Layout::from_size_align_unchecked(
			alloc_size,
			layout.align(),
		))
	}
}
