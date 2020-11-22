use crate::{registry::Component, storage::SparseSet};
use std::any::Any;

pub trait Storage
where
    Self: 'static,
{
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Storage for SparseSet<T>
where
    T: Component,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
