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
        self.remove_by_id(TypeId::of::<T>())
            .map(|r| match r.downcast::<T>() {
                Ok(r) => r,
                Err(_) => unsafe { unreachable_unchecked() },
            })
    }

    pub fn remove_by_id(&mut self, id: ResourceTypeId) -> Option<Box<dyn Any>> {
        self.values.remove(&id).map(|r| r.into_inner())
    }

    pub fn contains<T>(&self) -> bool
    where
        T: 'static,
    {
        self.contains_id(TypeId::of::<T>())
    }

    pub fn contains_id(&self, id: ResourceTypeId) -> bool {
        self.values.contains_key(&id)
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

    pub fn borrow_by_id(&self, id: ResourceTypeId) -> Option<Res<dyn Any>> {
        self.values
            .get(&id)
            .map(|r| Res(AtomicRef::map(r.borrow(), |r| r.as_ref())))
    }

    pub fn borrow_mut_by_id(&self, id: ResourceTypeId) -> Option<ResMut<dyn Any>> {
        self.values
            .get(&id)
            .map(|r| ResMut(AtomicRefMut::map(r.borrow_mut(), |r| r.as_mut())))
    }
}

pub struct Res<'a, T>(AtomicRef<'a, T>)
where
    T: 'static + ?Sized;

impl<T> Deref for Res<'_, T>
where
    T: 'static + ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ResMut<'a, T>(AtomicRefMut<'a, T>)
where
    T: 'static + ?Sized;

impl<T> Deref for ResMut<'_, T>
where
    T: 'static + ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ResMut<'_, T>
where
    T: 'static + ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
