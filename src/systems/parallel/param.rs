use crate::systems::{BorrowSystemData, LocalSystemParam};

/// Trait implemented by system parameters.
pub trait SystemParam: LocalSystemParam + for<'a> BorrowSystemData<'a> {
    // Empty
}

impl<T> SystemParam for T
where
    T: LocalSystemParam + for<'a> BorrowSystemData<'a>,
{
    // Empty
}
