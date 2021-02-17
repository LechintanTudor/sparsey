use crate::data::{AtomicRef, AtomicRefMut};
use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use std::ops::{Deref, DerefMut};

pub type ResourceTypeId = TypeId;

pub trait Resource
where
    Self: Downcast,
{
}

impl_downcast!(Resource);

pub struct Res<'a, T>(AtomicRef<'a, T>)
where
    T: ?Sized;

impl<'a, T> Res<'a, T>
where
    T: ?Sized,
{
    pub(crate) fn new(value: AtomicRef<'a, T>) -> Self {
        Self(value)
    }

    #[inline]
    pub fn clone(orig: &Self) -> Self {
        Self(AtomicRef::clone(&orig.0))
    }

    #[inline]
    pub fn map<U, F>(orig: Self, f: F) -> Res<'a, U>
    where
        F: FnOnce(&T) -> &U,
        U: ?Sized,
    {
        Res(AtomicRef::map(orig.0, f))
    }

    #[inline]
    pub fn filter_map<U, F>(orig: Self, f: F) -> Option<Res<'a, U>>
    where
        F: FnOnce(&T) -> Option<&U>,
        U: ?Sized,
    {
        AtomicRef::filter_map(orig.0, f).map(Res)
    }
}

impl<T> Deref for Res<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ResMut<'a, T>(AtomicRefMut<'a, T>)
where
    T: ?Sized;

impl<'a, T> ResMut<'a, T>
where
    T: ?Sized,
{
    pub(crate) fn new(value: AtomicRefMut<'a, T>) -> Self {
        Self(value)
    }

    #[inline]
    pub fn map<U, F>(orig: Self, f: F) -> ResMut<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: ?Sized,
    {
        ResMut(AtomicRefMut::map(orig.0, f))
    }

    #[inline]
    pub fn filter_map<U, F>(orig: Self, f: F) -> Option<ResMut<'a, U>>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
        U: ?Sized,
    {
        AtomicRefMut::filter_map(orig.0, f).map(ResMut)
    }
}

impl<T> Deref for ResMut<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ResMut<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
