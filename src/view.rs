use crate::Component;
use std::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ViewType {
    Read,
    Write,
    Maintain,
}

pub trait View {
    const TYPE: ViewType;
    type Component: Component;
}

/// Read components
pub struct Read<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<T> View for Read<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Read;
    type Component = T;
}

/// Write components
pub struct Write<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<T> View for Write<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Write;
    type Component = T;
}

/// Insert/delete components
/// Locks whole group.
pub struct Maintain<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<T> View for Maintain<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Maintain;
    type Component = T;
}
