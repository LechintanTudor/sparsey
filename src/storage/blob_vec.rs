use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr;
use std::ptr::NonNull;

/// Container for blobs of data with a given [`Layout`] and destructor.
pub(crate) struct BlobVec {
	layout: Layout,
	drop: unsafe fn(*mut u8),
	swap_space: NonNull<u8>,
	ptr: NonNull<u8>,
	cap: usize,
	len: usize,
}

impl BlobVec {
	/// Creates a new, empty `BlobVec` capable of holding items with the
	/// given `Layout` and destructor.
	pub unsafe fn new(layout: Layout, drop: unsafe fn(*mut u8)) -> Self {
		let ptr = NonNull::new_unchecked(layout.align() as *mut u8);

		if layout.size() == 0 {
			// Times 2 so we can use ptr::copy_nonoverlapping on ZST
			let swap_space = NonNull::new_unchecked((layout.align() * 2) as *mut u8);

			Self {
				layout,
				drop,
				swap_space,
				ptr,
				cap: usize::MAX,
				len: 0,
			}
		} else {
			let swap_space =
				NonNull::new(alloc(layout)).unwrap_or_else(|| handle_alloc_error(layout));

			Self {
				layout,
				drop,
				swap_space,
				ptr,
				cap: 0,
				len: 0,
			}
		}
	}

	/// Copies the item at the given address to the end of the vector.
	/// This will cause a reallocation if the vector is full.
	pub unsafe fn push(&mut self, item: *const u8) {
		if self.len == self.cap {
			self.grow();
		}

		ptr::copy(item, self.get_unchecked(self.len), self.layout.size());
		self.len += 1;
	}

	/// Replaces the item at `index` with the item at the given address.
	/// The destructor of the replaced item is not called.
	/// Returns the address of the replaced item which is valid until
	/// the next call to any of the vector's functions.
	pub unsafe fn set_and_forget_prev_unchecked(
		&mut self,
		index: usize,
		value: *const u8,
	) -> *mut u8 {
		ptr::copy_nonoverlapping(
			self.get_unchecked(index),
			self.swap_space.as_ptr(),
			self.layout.size(),
		);
		ptr::copy(value, self.get_unchecked(index), self.layout.size());
		self.swap_space.as_ptr()
	}

	/// Replaces the item at `index` with the item at the given address
	/// and calls the destructor for the replaced item..
	pub unsafe fn set_and_drop_prev_unchecked(&mut self, index: usize, value: *const u8) {
		(self.drop)(self.get_unchecked(index));
		ptr::copy(value, self.get_unchecked(index), self.layout.size());
	}

	/// Removes the item at the given `index` by swapping it with the item at
	/// the last position and decrementing the length. The destructor of the
	/// removed items is not called. Returns the address of the removed item
	/// which is valid until the next call to any of the vector's functions.
	pub unsafe fn swap_remove_and_forget_unchecked(&mut self, index: usize) -> *mut u8 {
		let last_index = self.len - 1;

		// Copy current element to swap space
		ptr::copy_nonoverlapping(
			self.get_unchecked(index),
			self.swap_space.as_ptr(),
			self.layout.size(),
		);

		// Overwrite current element with last element
		ptr::copy(
			self.get_unchecked(last_index),
			self.get_unchecked(index),
			self.layout.size(),
		);

		self.len = last_index;
		self.swap_space.as_ptr()
	}

	/// Removes the item at the given `index` by swapping it with the item at
	/// the last position and decrementing the length. The destructor of the
	/// removed items is called.
	pub unsafe fn swap_remove_and_drop_unchecked(&mut self, index: usize) {
		(self.drop)(self.get_unchecked(index));

		let last_index = self.len - 1;

		// Overwrite current element with last element
		ptr::copy(
			self.get_unchecked(last_index),
			self.get_unchecked(index),
			self.layout.size(),
		);

		self.len = last_index;
	}

	/// Swaps the items at the given positions.
	pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
		// Copy first element to swap space
		ptr::copy_nonoverlapping(
			self.get_unchecked(a),
			self.swap_space.as_ptr(),
			self.layout.size(),
		);

		// Overwrite first element with second element
		ptr::copy(
			self.get_unchecked(b),
			self.get_unchecked(a),
			self.layout.size(),
		);

		// Overwrite second element with first element
		ptr::copy_nonoverlapping(
			self.swap_space.as_ptr(),
			self.get_unchecked(b),
			self.layout.size(),
		);
	}

	/// Returns the address of the item at the given `index`.
	pub unsafe fn get_unchecked(&self, index: usize) -> *mut u8 {
		self.ptr.as_ptr().add(index * self.layout.size())
	}

	/// Returns a pointer to the buffer where the items are stored.
	pub fn as_ptr(&self) -> *mut u8 {
		self.ptr.as_ptr()
	}

	/// Returns the number of items in the vector.
	#[allow(dead_code)]
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns `true` if the vector contains no items.
	#[allow(dead_code)]
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Returns the number of items the vector can hold without reallocating.
	#[allow(dead_code)]
	pub fn capacity(&self) -> usize {
		self.cap
	}

	/// Calls the destructor for all items and sets the length of the vector to
	/// zero.
	pub fn clear(&mut self) {
		let len = self.len;
		self.len = 0;

		for i in 0..len {
			unsafe {
				(self.drop)(self.ptr.as_ptr().add(i * self.layout.size()));
			}
		}
	}

	/// Copies all elements to a larger buffer, increasing the capacity of the
	/// vector.
	fn grow(&mut self) {
		assert!(self.layout.size() != 0, "BlobVec is overfull");

		unsafe {
			let (new_ptr, new_layout, new_cap) = if self.cap == 0 {
				(alloc(self.layout), self.layout, 1)
			} else {
				let new_cap = 2 * self.cap;
				let new_layout =
					repeat_layout(&self.layout, new_cap).expect("Layout should be valid");

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

		if self.layout.size() != 0 {
			unsafe {
				dealloc(self.swap_space.as_ptr(), self.layout);
			}

			if self.cap != 0 {
				unsafe {
					dealloc(
						self.ptr.as_ptr(),
						repeat_layout(&self.layout, self.cap).expect("Layout should be valid"),
					);
				}
			}
		}
	}
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
