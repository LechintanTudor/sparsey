use crate::data::Component;
use downcast_rs::{impl_downcast, Downcast};
use std::ops::DerefMut;

pub unsafe trait TypeErasedVec
where
    Self: Send + Sync + Downcast + 'static,
{
    fn component_count(&self) -> usize;

    fn swap_delete_component(&mut self, index: usize);

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

    fn swap_delete_component(&mut self, index: usize) {
        Vec::swap_remove(self, index);
    }

    fn clear_components(&mut self) {
        Vec::clear(self);
    }

    fn swap_components(&mut self, a: usize, b: usize) {
        Vec::deref_mut(self).swap(a, b);
    }
}
