use crate::resources::{Res, ResMut, Resource, Resources};
use crate::storage::Component;
use crate::systems::GenericSystemParam;
use crate::world::{Comp, CompMut, Entities, World};

pub trait LocalSystemParam: GenericSystemParam {
    /// Borrows the local parameter.
    fn borrow<'a>(world: &'a World, resources: &'a Resources) -> Self::Param<'a>;
}

impl<'a> LocalSystemParam for Entities<'a> {
    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow_entities()
    }
}

impl<'a, T> LocalSystemParam for Comp<'a, T>
where
    T: Component,
{
    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow()
    }
}

impl<'a, T> LocalSystemParam for CompMut<'a, T>
where
    T: Component,
{
    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow_mut()
    }
}

impl<'a, T> LocalSystemParam for Res<'a, T>
where
    T: Resource,
{
    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.borrow()
    }
}

impl<'a, T> LocalSystemParam for ResMut<'a, T>
where
    T: Resource,
{
    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.borrow_mut()
    }
}

impl<'a, T> LocalSystemParam for Option<Res<'a, T>>
where
    T: Resource,
{
    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.try_borrow()
    }
}

impl<'a, T> LocalSystemParam for Option<ResMut<'a, T>>
where
    T: Resource,
{
    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.try_borrow_mut()
    }
}
