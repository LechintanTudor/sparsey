use crate::{Entity, SparseArray, SparseSet};
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

const EXCLUSIVE_BIT: usize = !(usize::MAX >> 1);

#[derive(Copy, Clone, Default, Debug)]
pub struct InvalidBorrow;

pub struct AtomicRefCell<T>
where
    T: ?Sized,
{
    borrow: AtomicUsize,
    value: UnsafeCell<T>,
}

unsafe impl<T> Send for AtomicRefCell<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for AtomicRefCell<T> where T: ?Sized + Sync {}

impl<T> AtomicRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            borrow: AtomicUsize::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T> AtomicRefCell<T>
where
    T: ?Sized,
{
    pub fn borrow(&self) -> Ref<T> {
        self.try_borrow().unwrap()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.try_borrow_mut().unwrap()
    }

    pub fn try_borrow(&self) -> Result<Ref<T>, InvalidBorrow> {
        let guard = BorrowGuard::new(&self.borrow)?;

        Ok(Ref {
            guard,
            value: unsafe { &*self.value.get() },
        })
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, InvalidBorrow> {
        let guard = BorrowMutGuard::new(&self.borrow)?;

        Ok(RefMut {
            guard,
            value: unsafe { &mut *self.value.get() },
        })
    }

    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

pub struct Ref<'a, T>
where
    T: ?Sized,
{
    guard: BorrowGuard<'a>,
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    pub fn map<U, F>(self, f: F) -> Ref<'a, U>
    where
        U: ?Sized,
        F: FnOnce(&T) -> &U,
    {
        Ref {
            guard: self.guard,
            value: f(self.value),
        }
    }

    pub fn map_split<U, V, F>(self, f: F) -> (Ref<'a, U>, Ref<'a, V>)
    where
        U: ?Sized,
        V: ?Sized,
        F: FnOnce(&T) -> (&U, &V),
    {
        todo!()
    }
}

impl<T> Clone for Ref<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            guard: self.guard.clone(),
            value: self.value,
        }
    }
}

impl<T> Deref for Ref<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

pub struct RefMut<'a, T>
where
    T: ?Sized,
{
    guard: BorrowMutGuard<'a>,
    value: &'a mut T,
}

impl<'a, T> RefMut<'a, T> {
    pub fn map<U, F>(self, f: F) -> RefMut<'a, U>
    where
        U: ?Sized,
        F: FnOnce(&mut T) -> &mut U,
    {
        RefMut {
            guard: self.guard,
            value: f(self.value),
        }
    }

    pub fn map_split<U, V, F>(self, f: F) -> (RefMut<'a, U>, RefMut<'a, V>)
    where
        U: ?Sized,
        V: ?Sized,
        F: FnOnce(&mut T) -> (&mut U, &mut V),
    {
        todo!()
    }
}

impl<T> Deref for RefMut<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> DerefMut for RefMut<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

struct BorrowGuard<'a> {
    borrow: &'a AtomicUsize,
}

impl<'a> BorrowGuard<'a> {
    fn new(borrow: &'a AtomicUsize) -> Result<Self, InvalidBorrow> {
        let new_borrow = borrow.fetch_add(1, Ordering::Acquire) + 1;

        if new_borrow & EXCLUSIVE_BIT != 0 {
            if new_borrow != EXCLUSIVE_BIT {
                // Already mutably borrowed
                borrow.store(EXCLUSIVE_BIT, Ordering::Release);
            } else {
                // Too many immutable borrows
                // Will not happen in a real program
                borrow.fetch_sub(1, Ordering::Release);
            }

            return Err(InvalidBorrow);
        }

        Ok(Self { borrow })
    }
}

impl Clone for BorrowGuard<'_> {
    fn clone(&self) -> Self {
        Self::new(self.borrow).unwrap()
    }
}

impl Drop for BorrowGuard<'_> {
    fn drop(&mut self) {
        self.borrow.fetch_sub(1, Ordering::Release);
    }
}

struct BorrowMutGuard<'a> {
    borrow: &'a AtomicUsize,
}

impl<'a> BorrowMutGuard<'a> {
    fn new(borrow: &'a AtomicUsize) -> Result<Self, InvalidBorrow> {
        let old_borrow =
            match borrow.compare_exchange(0, EXCLUSIVE_BIT, Ordering::Acquire, Ordering::Relaxed) {
                Ok(value) => value,
                Err(value) => value,
            };

        if old_borrow != 0 {
            Err(InvalidBorrow)
        } else {
            Ok(Self { borrow })
        }
    }
}

impl Drop for BorrowMutGuard<'_> {
    fn drop(&mut self) {
        self.borrow.store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_shared_borrows() {
        let value = AtomicRefCell::new(100);
        let _b1 = value.borrow();
        let _b2 = value.borrow();
    }

    #[test]
    #[should_panic]
    fn unique_exclusive_borrows() {
        let value = AtomicRefCell::new(100);
        let _b1 = value.borrow_mut();
        let _b2 = value.borrow_mut();
    }

    #[test]
    #[should_panic]
    fn mixed_borrows() {
        let value = AtomicRefCell::new(100);
        let _b1 = value.borrow();
        let _b2 = value.borrow_mut();
    }
}
