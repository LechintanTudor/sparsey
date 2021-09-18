use std::ops::{Bound, RangeBounds};

#[inline]
pub(crate) fn range_to_bounds<R>(range: &R) -> (Bound<usize>, Bound<usize>)
where
    R: RangeBounds<usize>,
{
    (range.start_bound().cloned(), range.end_bound().cloned())
}
