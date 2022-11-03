use crate::layout::ComponentInfo;
use crate::resources::{Res, ResMut, Resource, Resources};
use crate::storage::Component;
use crate::world::{Comp, CompMut, Entities, World};
use std::any::TypeId;

/// The type of parameters a system can have.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SystemParamType {
    /// View over all entities.
    Entities,
    /// View over all components of a type.
    Comp(ComponentInfo),
    /// Mutable view over all components of a type.
    CompMut(ComponentInfo),
    /// View over a resource.
    Res(TypeId),
    /// Mutable view over a resource.
    ResMut(TypeId),
}

impl SystemParamType {
    /// Returns `true` if the parameters prevent the systems from running in parallel.
    pub fn conflicts_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Comp(c1), Self::CompMut(c2)) => c1 == c2,
            (Self::CompMut(c1), Self::Comp(c2)) => c1 == c2,
            (Self::CompMut(c1), Self::CompMut(c2)) => c1 == c2,
            (Self::Res(r1), Self::ResMut(r2)) => r1 == r2,
            (Self::ResMut(r1), Self::Res(r2)) => r1 == r2,
            (Self::ResMut(r1), Self::ResMut(r2)) => r1 == r2,
            _ => false,
        }
    }
}

pub trait LocalSystemParam {
    /// Type of parameter.
    type Param<'a>;

    /// Returns the type of parameter.
    fn param_type() -> SystemParamType;

    /// Borrows the local parameter.
    fn borrow<'a>(world: &'a World, resources: &'a Resources) -> Self::Param<'a>;
}

impl<'a> LocalSystemParam for Entities<'a> {
    type Param<'b> = Entities<'b>;

    fn param_type() -> SystemParamType {
        SystemParamType::Entities
    }

    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow_entities()
    }
}

impl<'a, T> LocalSystemParam for Comp<'a, T>
where
    T: Component,
{
    type Param<'b> = Comp<'b, T>;

    fn param_type() -> SystemParamType {
        SystemParamType::Comp(ComponentInfo::new::<T>())
    }

    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow()
    }
}

impl<'a, T> LocalSystemParam for CompMut<'a, T>
where
    T: Component,
{
    type Param<'b> = CompMut<'b, T>;

    fn param_type() -> SystemParamType {
        SystemParamType::CompMut(ComponentInfo::new::<T>())
    }

    fn borrow<'b>(world: &'b World, _resources: &'b Resources) -> Self::Param<'b> {
        world.borrow_mut()
    }
}

impl<'a, T> LocalSystemParam for Res<'a, T>
where
    T: Resource,
{
    type Param<'b> = Res<'b, T>;

    fn param_type() -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }

    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.borrow()
    }
}

impl<'a, T> LocalSystemParam for ResMut<'a, T>
where
    T: Resource,
{
    type Param<'b> = ResMut<'b, T>;

    fn param_type() -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }

    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.borrow_mut()
    }
}

impl<'a, T> LocalSystemParam for Option<Res<'a, T>>
where
    T: Resource,
{
    type Param<'b> = Option<Res<'b, T>>;

    fn param_type() -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }

    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.try_borrow()
    }
}

impl<'a, T> LocalSystemParam for Option<ResMut<'a, T>>
where
    T: Resource,
{
    type Param<'b> = Option<ResMut<'b, T>>;

    fn param_type() -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }

    fn borrow<'b>(_world: &'b World, resources: &'b Resources) -> Self::Param<'b> {
        resources.try_borrow_mut()
    }
}
