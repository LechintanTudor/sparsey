use crate::resources::{Res, ResMut, Resource, Resources};
use crate::storage::Component;
use crate::systems::SystemBorrow;
use crate::utils::{ComponentData, ResourceData};
use crate::world::{Comp, CompMut, Entities, World};

/// Trait implemented by parameters of functions used to create
/// [`LocalSystems`](crate::systems::LocalSystem).
pub trait LocalSystemParam {
    /// Parameter type generic over its lifetime.
    type Param<'a>;

    /// Returns the [`SystemBorrow`] associated with this type.
    fn as_system_borrow() -> SystemBorrow;

    /// Borrows the parameter.
    fn borrow<'a>(world: &'a World, resources: &'a Resources) -> Self::Param<'a>;
}

impl LocalSystemParam for Entities<'_> {
    type Param<'a> = Entities<'a>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::Entities
    }

    fn borrow<'a>(world: &'a World, _resources: &'a Resources) -> Self::Param<'a> {
        world.borrow_entities()
    }
}

impl<T> LocalSystemParam for Comp<'_, T>
where
    T: Component,
{
    type Param<'a> = Comp<'a, T>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::Comp(ComponentData::new::<T>())
    }

    fn borrow<'a>(world: &'a World, _resources: &'a Resources) -> Self::Param<'a> {
        world.borrow()
    }
}

impl<T> LocalSystemParam for CompMut<'_, T>
where
    T: Component,
{
    type Param<'a> = CompMut<'a, T>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::CompMut(ComponentData::new::<T>())
    }

    fn borrow<'a>(world: &'a World, _resources: &'a Resources) -> Self::Param<'a> {
        world.borrow_mut()
    }
}

impl<T> LocalSystemParam for Res<'_, T>
where
    T: Resource,
{
    type Param<'a> = Res<'a, T>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::Res(ResourceData::new::<T>())
    }

    fn borrow<'a>(_world: &'a World, resources: &'a Resources) -> Self::Param<'a> {
        resources.borrow()
    }
}

impl<T> LocalSystemParam for ResMut<'_, T>
where
    T: Resource,
{
    type Param<'a> = ResMut<'a, T>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::ResMut(ResourceData::new::<T>())
    }

    fn borrow<'a>(_world: &'a World, resources: &'a Resources) -> Self::Param<'a> {
        resources.borrow_mut()
    }
}

impl<T> LocalSystemParam for Option<Res<'_, T>>
where
    T: Resource,
{
    type Param<'a> = Option<Res<'a, T>>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::Res(ResourceData::new::<T>())
    }

    fn borrow<'a>(_world: &'a World, resources: &'a Resources) -> Self::Param<'a> {
        resources.try_borrow()
    }
}

impl<T> LocalSystemParam for Option<ResMut<'_, T>>
where
    T: Resource,
{
    type Param<'a> = Option<ResMut<'a, T>>;

    fn as_system_borrow() -> SystemBorrow {
        SystemBorrow::ResMut(ResourceData::new::<T>())
    }

    fn borrow<'a>(_world: &'a World, resources: &'a Resources) -> Self::Param<'a> {
        resources.try_borrow_mut()
    }
}
