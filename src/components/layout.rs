use std::alloc::Layout;

// From https://doc.rust-lang.org/src/core/alloc/layout.rs.html
pub fn repeat_layout(layout: &Layout, n: usize) -> Option<Layout> {
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
