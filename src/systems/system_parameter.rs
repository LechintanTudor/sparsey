use crate::components::Component;
use crate::resources::{Res, ResMut, Resource};
use crate::systems::{BorrowRegistry, Commands};
use crate::world::{Comp, CompMut};

/// Trait used for marking system parameters and borrowing data from the
/// `Registry`.
pub trait LocalSystemParam
where
	Self: for<'a> BorrowRegistry<'a>,
{
	// Empty
}

/// Marker trait for parameters of systems which are safe to run
/// from threads other than the one in which they were created.
pub unsafe trait SystemParam
where
	Self: LocalSystemParam,
{
	// Empty
}

impl<'a, T> LocalSystemParam for Comp<'a, T>
where
	T: Component,
{
	// Empty
}

unsafe impl<'a, T> SystemParam for Comp<'a, T>
where
	T: Component,
{
	// Empty
}

impl<'a, T> LocalSystemParam for CompMut<'a, T>
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

impl<'a, T> LocalSystemParam for Res<'a, T>
where
	T: Resource,
{
	// Empty
}

unsafe impl<'a, T> SystemParam for Res<'a, T>
where
	T: Resource + Sync,
{
	// Empty
}

impl<'a, T> LocalSystemParam for ResMut<'a, T>
where
	T: Resource + Send,
{
	// Empty
}

unsafe impl<'a, T> SystemParam for ResMut<'a, T>
where
	T: Resource + Send,
{
	// Empty
}

impl<'a> LocalSystemParam for Commands<'a> {
	// Empty
}

unsafe impl<'a> SystemParam for Commands<'a> {
	// Empty
}
