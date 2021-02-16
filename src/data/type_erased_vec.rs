use crate::data::Component;
use downcast_rs::{impl_downcast, Downcast};
use std::ops::DerefMut;

pub trait TypeErasedVec
where
    Self: Downcast,
{
    fn len(&self) -> usize;

    fn clear(&mut self);

    fn swap(&mut self, a: usize, b: usize);
}

impl_downcast!(TypeErasedVec);

impl<T> TypeErasedVec for Vec<T>
where
    T: Component,
{
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn clear(&mut self) {
        Vec::clear(self);
    }

    fn swap(&mut self, a: usize, b: usize) {
        Vec::deref_mut(self).swap(a, b);
    }
}
