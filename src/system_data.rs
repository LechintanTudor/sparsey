use crate::{Component, Resources, World};

pub trait SystemData {
    fn fetch(world: &World, resources: &Resources) -> Self;
}

/*
pub struct Comp<T>
where
    T: Component;

impl<T> SystemData for Comp<T> 
where 
    T: Component,
{
    fn fetch(world: &World, resources: &Resources) -> Self {
        Self
    }
}

pub struct CompMut<T>
where
    T: Component;

impl<T> SystemData for CompMut<T> 
where 
    T: Component,
{
    fn fetch(world: &World, resources: &Resources) -> Self {
        Self
    }
}

pub struct Res;
*/
