use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub type ResourceTypeId = TypeId;

#[derive(Default)]
pub struct Resources {
    values: HashMap<ResourceTypeId, AtomicRefCell<Box<dyn Any>>>,
    _non_send_sync: PhantomData<*const ()>,
}

impl Resources {
    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
    where
        T: 'static,
    {
        self.insert_boxed(Box::new(resource))
    }

    pub fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
    where
        T: 'static,
    {
        self.values
            .insert(TypeId::of::<T>(), AtomicRefCell::new(resource))
            .map(|r| match r.into_inner().downcast::<T>() {
                Ok(r) => r,
                Err(_) => unsafe { unreachable_unchecked() },
            })
    }

    pub fn remove<T>(&mut self) -> Option<Box<T>>
    where
        T: 'static,
    {
        self.values
            .remove(&TypeId::of::<T>())
            .map(|r| match r.into_inner().downcast::<T>() {
                Ok(r) => r,
                Err(_) => unsafe { unreachable_unchecked() },
            })
    }

    pub fn borrow<T>(&self) -> Option<Res<T>>
    where
        T: 'static,
    {
        self.values.get(&TypeId::of::<T>()).map(|r| {
            Res(AtomicRef::map(r.borrow(), |r| {
                match r.as_ref().downcast_ref::<T>() {
                    Some(r) => r,
                    None => unsafe { unreachable_unchecked() },
                }
            }))
        })
    }

    pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: 'static,
    {
        self.values.get(&TypeId::of::<T>()).map(|r| {
            ResMut(AtomicRefMut::map(r.borrow_mut(), |r| {
                match r.as_mut().downcast_mut::<T>() {
                    Some(r) => r,
                    None => unsafe { unreachable_unchecked() },
                }
            }))
        })
    }
}

pub struct Res<'a, T>(AtomicRef<'a, T>)
where
    T: 'static;

impl<T> Deref for Res<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ResMut<'a, T>(AtomicRefMut<'a, T>)
where
    T: 'static;

impl<T> Deref for ResMut<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ResMut<'_, T>
where
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
