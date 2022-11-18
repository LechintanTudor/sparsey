use crate::resources::{Res, ResMut, Resource, SyncResources};
use crate::storage::Component;
use crate::systems::LocalSystemParam;
use crate::world::{Comp, CompMut, Entities, World};

/// Trait implemented by system parameters.
pub trait SystemParam: LocalSystemParam {
    /// Borrows the parameter.
    fn borrow<'a>(world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a>;
}

impl<'a> SystemParam for Entities<'a> {
    fn borrow<'b>(world: &'b World, _resources: SyncResources<'b>) -> Self::Param<'b> {
        world.borrow_entities()
    }
}

impl<'a, T> SystemParam for Comp<'a, T>
where
    T: Component,
{
    fn borrow<'b>(world: &'b World, _resources: SyncResources<'b>) -> Self::Param<'b> {
        world.borrow()
    }
}

impl<'a, T> SystemParam for CompMut<'a, T>
where
    T: Component,
{
    fn borrow<'b>(world: &'b World, _resources: SyncResources<'b>) -> Self::Param<'b> {
        world.borrow_mut()
    }
}

impl<'a, T> SystemParam for Res<'a, T>
where
    T: Resource + Sync,
{
    fn borrow<'b>(_world: &'b World, resources: SyncResources<'b>) -> Self::Param<'b> {
        resources.borrow()
    }
}

impl<'a, T> SystemParam for ResMut<'a, T>
where
    T: Resource + Send,
{
    fn borrow<'b>(_world: &'b World, resources: SyncResources<'b>) -> Self::Param<'b> {
        resources.borrow_mut()
    }
}

impl<'a, T> SystemParam for Option<Res<'a, T>>
where
    T: Resource + Sync,
{
    fn borrow<'b>(_world: &'b World, resources: SyncResources<'b>) -> Self::Param<'b> {
        resources.try_borrow()
    }
}

impl<'a, T> SystemParam for Option<ResMut<'a, T>>
where
    T: Resource + Send,
{
    fn borrow<'b>(_world: &'b World, resources: SyncResources<'b>) -> Self::Param<'b> {
        resources.try_borrow_mut()
    }
}
