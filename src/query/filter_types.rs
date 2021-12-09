/// Filter that matches all inputs.
#[derive(Clone, Copy, Default, Debug)]
pub struct Passthrough;

/// Filter that only matches newly added components.
#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

/// Filter that only matches components which were mutated since the last check.
#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

/// Filter that only matches added or mutated components.
#[derive(Clone, Copy, Default, Debug)]
pub struct Changed;

/// Filter wrapper that negates the result of the inner filter.
pub struct Not<F>(pub(crate) F);

/// Filter wrapper that "ands" the result of the 2 inner filters.
pub struct And<F1, F2>(pub(crate) F1, pub(crate) F2);

/// Filter wrapper that "ors" the result of the 2 inner filters.
pub struct Or<F1, F2>(pub(crate) F1, pub(crate) F2);

/// Filter wrapper that "xors" the result of the 2 inner filters.
pub struct Xor<F1, F2>(pub(crate) F1, pub(crate) F2);
