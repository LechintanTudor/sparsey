use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr;
use std::ptr::NonNull;

const MIN_NON_ZERO_CAP: usize = 4;

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
        let (swap_space, cap) = if layout.size() == 0 {
            (
                NonNull::new_unchecked(layout.align() as *mut u8),
                usize::MAX,
            )
        } else {
            (
                NonNull::new(alloc(layout)).unwrap_or_else(|| handle_alloc_error(layout)),
                0,
            )
        };

        Self {
            layout,
            drop,
            swap_space,
            ptr: NonNull::new_unchecked(layout.align() as *mut u8),
            cap,
            len: 0,
        }
    }

    /// Copies the value at the given address to the end of the vector.
    /// This will cause a reallocation if the vector is full.
    pub unsafe fn push(&mut self, value: *const u8) {
        if self.len == self.cap {
            self.grow_amortized();
        }

        let size = self.layout.size();
        let slot = self.ptr.as_ptr().add(self.len * size);

        ptr::copy_nonoverlapping(value, slot, size);
        self.len += 1;
    }

    /// Replaces the item at `index` with the value at the given address.
    /// The destructor of the replaced item is not called.
    /// Returns the address of the replaced item which remains valid until
    /// the next call to any of the vector's functions.
    pub unsafe fn set_and_forget_prev_unchecked(
        &mut self,
        index: usize,
        value: *const u8,
    ) -> *mut u8 {
        debug_assert!(index < self.len);

        let size = self.layout.size();
        let to_remove = self.ptr.as_ptr().add(index * size);
        let swap_space = self.swap_space.as_ptr();

        ptr::copy_nonoverlapping(to_remove, swap_space, size);
        ptr::copy_nonoverlapping(value, to_remove, size);
        swap_space
    }

    /// Replaces the item at `index` with the value at the given address
    /// and calls the destructor for the replaced item.
    pub unsafe fn set_and_drop_prev_unchecked(&mut self, index: usize, value: *const u8) {
        debug_assert!(index < self.len);

        let size = self.layout.size();
        let to_remove = self.ptr.as_ptr().add(index * size);

        (self.drop)(to_remove);
        ptr::copy_nonoverlapping(value, to_remove, size);
    }

    /// Removes the item at `index` by swapping it with the item at
    /// the last position and decrementing the length. The destructor of the
    /// removed items is not called. Returns the address of the removed item
    /// which remains valid until the next call to any of the vector's
    /// functions.
    pub unsafe fn swap_remove_and_forget_unchecked(&mut self, index: usize) -> *mut u8 {
        debug_assert!(index < self.len);

        self.len -= 1;

        let size = self.layout.size();
        let to_remove = self.ptr.as_ptr().add(index * size);
        let swap_space = self.swap_space.as_ptr();
        let last = self.ptr.as_ptr().add(self.len * size);

        ptr::copy_nonoverlapping(to_remove, swap_space, size);
        ptr::copy(last, to_remove, size);
        swap_space
    }

    /// Removes the item at the given `index` by swapping it with the item at
    /// the last position and decrementing the length. The destructor of the
    /// removed item is called.
    pub unsafe fn swap_remove_and_drop_unchecked(&mut self, index: usize) {
        debug_assert!(index < self.len);

        self.len -= 1;

        let size = self.layout.size();
        let to_remove = self.ptr.as_ptr().add(index * size);
        let last = self.ptr.as_ptr().add(self.len * size);

        (self.drop)(to_remove);
        ptr::copy(last, to_remove, size);
    }

    /// Swaps the items at the given positions without checking if the positions
    /// are valid.
    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.len);
        debug_assert!(b < self.len);

        let size = self.layout.size();
        let a = self.ptr.as_ptr().add(a * size);
        let b = self.ptr.as_ptr().add(b * size);
        let swap_space = self.swap_space.as_ptr();

        ptr::copy_nonoverlapping(a, swap_space, size);
        ptr::copy(b, a, size);
        ptr::copy_nonoverlapping(swap_space, b, size);
    }

    /// Returns the address of the item at the given `index`.
    pub unsafe fn get_unchecked(&self, index: usize) -> *const u8 {
        debug_assert!(index < self.len);
        self.ptr.as_ptr().add(index * self.layout.size())
    }

    /// Returns the address of the item at the given `index`.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> *mut u8 {
        debug_assert!(index < self.len);
        self.ptr.as_ptr().add(index * self.layout.size())
    }

    /// Returns a pointer to the buffer where the items are stored.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Returns a pointer to the buffer where the items are stored.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Returns the number of items in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no items.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of items the vector can hold without reallocating.
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
    fn grow_amortized(&mut self) {
        assert!(self.layout.size() != 0, "BlobVec is overfull");

        unsafe {
            let (new_ptr, new_layout, new_cap) = if self.cap == 0 {
                let new_cap = MIN_NON_ZERO_CAP;
                let new_layout =
                    repeat_layout(&self.layout, new_cap).expect("Layout should be valid");

                (alloc(new_layout), new_layout, new_cap)
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
