use downcast_rs::{impl_downcast, Downcast};

/// Trait automatically implemented by all types which
/// can be stored in `Resources`.
pub trait Resource
where
    Self: Downcast,
{
}

impl_downcast!(Resource);

impl<T> Resource for T where T: Downcast {}
