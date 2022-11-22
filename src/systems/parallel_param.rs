use crate::resources::{Res, ResMut, Resource, SyncResources};
use crate::storage::Component;
use crate::systems::LocalSystemParam;
use crate::world::{Comp, CompMut, Entities, World};

/// Trait implemented by parameters of functions used to create [`Systems`](crate::systems::System).
pub trait SystemParam: LocalSystemParam {
    /// Borrows the parameter.
    fn borrow<'a>(world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a>;
}

impl SystemParam for Entities<'_> {
    fn borrow<'a>(world: &'a World, _resources: SyncResources<'a>) -> Self::Param<'a> {
        world.borrow_entities()
    }
}

impl<T> SystemParam for Comp<'_, T>
where
    T: Component,
{
    fn borrow<'a>(world: &'a World, _resources: SyncResources<'a>) -> Self::Param<'a> {
        world.borrow()
    }
}

impl<T> SystemParam for CompMut<'_, T>
where
    T: Component,
{
    fn borrow<'a>(world: &'a World, _resources: SyncResources<'a>) -> Self::Param<'a> {
        world.borrow_mut()
    }
}

impl<T> SystemParam for Res<'_, T>
where
    T: Resource + Sync,
{
    fn borrow<'a>(_world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a> {
        resources.borrow()
    }
}

impl<T> SystemParam for ResMut<'_, T>
where
    T: Resource + Send,
{
    fn borrow<'a>(_world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a> {
        resources.borrow_mut()
    }
}

impl<T> SystemParam for Option<Res<'_, T>>
where
    T: Resource + Sync,
{
    fn borrow<'a>(_world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a> {
        resources.try_borrow()
    }
}

impl<T> SystemParam for Option<ResMut<'_, T>>
where
    T: Resource + Send,
{
    fn borrow<'a>(_world: &'a World, resources: SyncResources<'a>) -> Self::Param<'a> {
        resources.try_borrow_mut()
    }
}
