use crate::components::Component;
use crate::resources::Resource;
use crate::systems::{BorrowRegistry, Commands};
use crate::world::{Comp, CompMut, Res, ResMut};

/// Trait used for marking system parameters and borrowing data from the
/// `Registry`.
pub trait LocalSystemParam: for<'a> BorrowRegistry<'a> {
    // Empty
}

impl<T> LocalSystemParam for T
where
    T: for<'a> BorrowRegistry<'a>,
{
    // Empty
}

/// Marker trait for parameters of systems which are safe to run
/// from threads other than the one in which they were created.
pub unsafe trait SystemParam: LocalSystemParam {
    // Empty
}

unsafe impl<'a, T> SystemParam for Comp<'a, T>
where
    T: Component,
{
    // Empty
}

unsafe impl<'a, T> SystemParam for CompMut<'a, T>
where
    T: Component,
{
    // Empty
}

unsafe impl<'a, T> SystemParam for Res<'a, T>
where
    T: Resource + Sync,
{
    // Empty
}

unsafe impl<'a, T> SystemParam for ResMut<'a, T>
where
    T: Resource + Send,
{
    // Empty
}

unsafe impl<'a, T> SystemParam for Option<Res<'a, T>>
where
    T: Resource + Sync,
{
    // Empty
}

unsafe impl<'a, T> SystemParam for Option<ResMut<'a, T>>
where
    T: Resource + Send,
{
    // Empty
}

unsafe impl<'a> SystemParam for Commands<'a> {
    // Empty
}
