use crate::SparseSet;
use std::any::Any;

pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

pub struct Storage<T>
where
    T: Component,
{
    storage: SparseSet<T>,
}

impl<T> Default for Storage<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}

pub trait GenericStorage {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> GenericStorage for Storage<T>
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
