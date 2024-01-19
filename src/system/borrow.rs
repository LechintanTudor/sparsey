use crate::entity::{Comp, CompMut, Component, Entities, EntityStorage};
use crate::resource::{Res, ResMut, Resource, ResourceStorage};
use crate::system::SystemParam;
use crate::World;

/// Helper trait for borrowing data from a registry.
pub trait SystemBorrow<TRegistry = World>: SystemParam {
    /// Borrows data from the given `registry`.
    #[must_use]
    fn borrow(registry: &TRegistry) -> Self::Param<'_>;
}

impl SystemBorrow<World> for Entities<'_> {
    fn borrow(world: &World) -> Self::Param<'_> {
        world.entities.borrow_entities()
    }
}

impl<T> SystemBorrow<World> for Comp<'_, T>
where
    T: Component,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.entities.borrow()
    }
}

impl<T> SystemBorrow<World> for CompMut<'_, T>
where
    T: Component,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.entities.borrow_mut()
    }
}

impl<T> SystemBorrow<World> for Res<'_, T>
where
    T: Resource,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.resources.borrow()
    }
}

impl<T> SystemBorrow<World> for ResMut<'_, T>
where
    T: Resource,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.resources.borrow_mut()
    }
}

impl<T> SystemBorrow<World> for Option<Res<'_, T>>
where
    T: Resource,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.resources.try_borrow()
    }
}

impl<T> SystemBorrow<World> for Option<ResMut<'_, T>>
where
    T: Resource,
{
    fn borrow(world: &World) -> Self::Param<'_> {
        world.resources.try_borrow_mut()
    }
}

impl SystemBorrow<EntityStorage> for Entities<'_> {
    fn borrow(entities: &EntityStorage) -> Self::Param<'_> {
        entities.borrow_entities()
    }
}

impl<T> SystemBorrow<EntityStorage> for Comp<'_, T>
where
    T: Component,
{
    fn borrow(entities: &EntityStorage) -> Self::Param<'_> {
        entities.borrow()
    }
}

impl<T> SystemBorrow<EntityStorage> for CompMut<'_, T>
where
    T: Component,
{
    fn borrow(entities: &EntityStorage) -> Self::Param<'_> {
        entities.borrow_mut()
    }
}

impl<T> SystemBorrow<ResourceStorage> for Res<'_, T>
where
    T: Resource,
{
    fn borrow(resources: &ResourceStorage) -> Self::Param<'_> {
        resources.borrow()
    }
}

impl<T> SystemBorrow<ResourceStorage> for ResMut<'_, T>
where
    T: Resource,
{
    fn borrow(resources: &ResourceStorage) -> Self::Param<'_> {
        resources.borrow_mut()
    }
}

impl<T> SystemBorrow<ResourceStorage> for Option<Res<'_, T>>
where
    T: Resource,
{
    fn borrow(resources: &ResourceStorage) -> Self::Param<'_> {
        resources.try_borrow()
    }
}

impl<T> SystemBorrow<ResourceStorage> for Option<ResMut<'_, T>>
where
    T: Resource,
{
    fn borrow(resources: &ResourceStorage) -> Self::Param<'_> {
        resources.try_borrow_mut()
    }
}
