use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// Shared view over a [`Resource`](crate::resources::Resource).
pub struct Res<'a, T>(AtomicRef<'a, T>);

impl<'a, T> Res<'a, T> {
    pub(crate) fn new(resource: AtomicRef<'a, T>) -> Self {
        Self(resource)
    }
}

/// Exclusive view over a [`Resource`](crate::resources::Resource).
pub struct ResMut<'a, T>(AtomicRefMut<'a, T>);

impl<'a, T> ResMut<'a, T> {
    pub(crate) fn new(resource: AtomicRefMut<'a, T>) -> Self {
        Self(resource)
    }
}

impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

macro_rules! impl_res_common {
    ($Res:ident) => {
        impl<T> Deref for $Res<'_, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.0.deref()
            }
        }

        impl<T> fmt::Debug for $Res<'_, T>
        where
            T: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($Res))
                    .field(self.0.deref())
                    .finish()
            }
        }

        impl<T> fmt::Display for $Res<'_, T>
        where
            T: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self.0.deref(), f)
            }
        }
    };
}

impl_res_common!(Res);
impl_res_common!(ResMut);
