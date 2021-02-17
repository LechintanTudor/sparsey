use bitflags::bitflags;
use std::ops::{Deref, DerefMut};

bitflags! {
    pub struct ComponentFlags: u8 {
        const ADDED   = 0b00000001;
        const CHANGED = 0b00000010;
    }
}

pub struct ComponentRefMut<'a, T>
where
    T: 'static,
{
    data: &'a mut T,
    flags: &'a mut ComponentFlags,
}

impl<'a, T> ComponentRefMut<'a, T>
where
    T: 'static,
{
    pub fn new(data: &'a mut T, flags: &'a mut ComponentFlags) -> Self {
        Self { data, flags }
    }
}

impl<T> Deref for ComponentRefMut<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ComponentRefMut<'_, T>
where
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.flags.insert(ComponentFlags::CHANGED);
        self.data
    }
}
