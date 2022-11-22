use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::ops::{Deref, DerefMut};

/// Shared view over a [`Resource`](crate::resources::Resource).
pub struct Res<'a, T>(AtomicRef<'a, T>);

impl<'a, T> Res<'a, T> {
    pub(crate) fn new(resource: AtomicRef<'a, T>) -> Self {
        Self(resource)
    }
}

impl<T> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// Exclusive view over a [`Resource`](crate::resources::Resource).
pub struct ResMut<'a, T>(AtomicRefMut<'a, T>);

impl<'a, T> ResMut<'a, T> {
    pub(crate) fn new(resource: AtomicRefMut<'a, T>) -> Self {
        Self(resource)
    }
}

impl<T> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
