#[derive(Clone, Copy, Default, Debug)]
pub struct Passthrough;

#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

#[derive(Clone, Copy, Default, Debug)]
pub struct Changed;

pub struct Not<F>(pub(crate) F);

pub struct And<F1, F2>(pub(crate) F1, pub(crate) F2);

pub struct Or<F1, F2>(pub(crate) F1, pub(crate) F2);

pub struct Xor<F1, F2>(pub(crate) F1, pub(crate) F2);
