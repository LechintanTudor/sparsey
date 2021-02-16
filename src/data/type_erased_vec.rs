use crate::data::Component;
use downcast_rs::{impl_downcast, Downcast};
use std::ops::DerefMut;

pub unsafe trait TypeErasedVec
where
    Self: Downcast,
{
    fn component_count(&self) -> usize;

    fn clear_components(&mut self);

    fn swap_components(&mut self, a: usize, b: usize);
}

impl_downcast!(TypeErasedVec);

unsafe impl<T> TypeErasedVec for Vec<T>
where
    T: Component,
{
    fn component_count(&self) -> usize {
        Vec::len(self)
    }

    fn clear_components(&mut self) {
        Vec::clear(self);
    }

    fn swap_components(&mut self, a: usize, b: usize) {
        Vec::deref_mut(self).swap(a, b);
    }
}
