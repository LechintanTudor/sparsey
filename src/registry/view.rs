use crate::{
    data::{IterableView, ParentGroup},
    entity::Entity,
    registry::{Component, World},
    storage::{ComponentFlags, ComponentRefMut, SparseArray, SparseSet},
};
use std::any::TypeId;

pub trait ComponentView {
    type Component: 'static;
}

pub trait GetFromWorld<'a> {
    unsafe fn is_get_from_world_safe() -> bool;

    unsafe fn get_from_world(world: &'a World) -> Self;
}

#[derive(Copy, Clone)]
pub struct Comp<'a, T>
where
    T: 'static,
{
    set: &'a SparseSet<T>,
    group: Option<ParentGroup>,
}

impl<'a, T> Comp<'a, T> {
    pub unsafe fn new(set: &'a SparseSet<T>, group: Option<ParentGroup>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> IterableView<'a> for Comp<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(&*data.add(index))
    }
}

impl<T> ComponentView for Comp<'_, T>
where
    T: 'static,
{
    type Component = T;
}

impl<'a, T> GetFromWorld<'a> for Comp<'a, T>
where
    T: Component,
{
    unsafe fn is_get_from_world_safe() -> bool {
        true
    }

    unsafe fn get_from_world(world: &'a World) -> Self {
        world.get_comp().unwrap()
    }
}

pub struct CompMut<'a, T>
where
    T: 'static,
{
    set: &'a mut SparseSet<T>,
    group: Option<ParentGroup>,
}

impl<'a, T> CompMut<'a, T> {
    pub unsafe fn new(set: &'a mut SparseSet<T>, group: Option<ParentGroup>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> IterableView<'a> for &'a CompMut<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(&*data.add(index))
    }
}

impl<'a, T> IterableView<'a> for &'a mut CompMut<'a, T> {
    type Data = *mut T;
    type Flags = *mut ComponentFlags;
    type Output = ComponentRefMut<'a, T>;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split_mut();
        (sparse, dense, data.as_mut_ptr(), flags.as_mut_ptr())
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(ComponentRefMut::new(
            &mut *data.add(index),
            &mut *flags.add(index),
        ))
    }
}

impl<T> ComponentView for CompMut<'_, T>
where
    T: 'static,
{
    type Component = T;
}

impl<'a, T> GetFromWorld<'a> for CompMut<'a, T>
where
    T: Component,
{
    unsafe fn is_get_from_world_safe() -> bool {
        true
    }

    unsafe fn get_from_world(world: &'a World) -> Self {
        world.get_comp_mut().unwrap()
    }
}

macro_rules! impl_get_from_world {
    ($($comp:ident),+) => {
        impl<'a, $($comp,)+> GetFromWorld<'a> for ($($comp,)+)
        where
            $($comp: ComponentView + GetFromWorld<'a>,)+
        {
            unsafe fn is_get_from_world_safe() -> bool {
                let mut comps = vec![$(TypeId::of::<$comp::Component>(),)+];
                let initial_len = comps.len();

                comps.sort_unstable();
                comps.dedup();
                comps.len() == initial_len
            }

            unsafe fn get_from_world(world: &'a World) -> Self {
                (
                    $($comp::get_from_world(world),)+
                )
            }
        }
    };
}

impl_get_from_world!(A);
impl_get_from_world!(A, B);
impl_get_from_world!(A, B, C);
impl_get_from_world!(A, B, C, D);
impl_get_from_world!(A, B, C, D, E);
impl_get_from_world!(A, B, C, D, E, F);
impl_get_from_world!(A, B, C, D, E, F, G);
impl_get_from_world!(A, B, C, D, E, F, G, H);
impl_get_from_world!(A, B, C, D, E, F, G, H, I);
impl_get_from_world!(A, B, C, D, E, F, G, H, I, J);
impl_get_from_world!(A, B, C, D, E, F, G, H, I, J, K);
impl_get_from_world!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::*;

    struct A;
    struct B;
    struct C;

    #[test]
    fn is_get_from_world_safe() {
        unsafe {
            assert!(<(Comp<A>, Comp<B>, Comp<C>)>::is_get_from_world_safe());
            assert!(<(Comp<A>, Comp<B>, Comp<A>)>::is_get_from_world_safe() == false);
        }
    }
}
