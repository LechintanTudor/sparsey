use crate::layout::ComponentInfo;
use crate::resources::{Res, ResMut, Resource};
use crate::storage::Component;
use crate::systems::BorrowLocalSystemData;
use crate::world::{Comp, CompMut, Entities};
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

/// Trait implemented by local system parameters.
pub trait LocalSystemParam: for<'a> BorrowLocalSystemData<'a> {
    /// Returns the type of parameter.
    fn param_type() -> SystemParamType;
}

impl<'a> LocalSystemParam for Entities<'a> {
    fn param_type() -> SystemParamType {
        SystemParamType::Entities
    }
}

impl<'a, T> LocalSystemParam for Comp<'a, T>
where
    T: Component,
{
    fn param_type() -> SystemParamType {
        SystemParamType::Comp(ComponentInfo::new::<T>())
    }
}

impl<'a, T> LocalSystemParam for CompMut<'a, T>
where
    T: Component,
{
    fn param_type() -> SystemParamType {
        SystemParamType::CompMut(ComponentInfo::new::<T>())
    }
}

impl<'a, T> LocalSystemParam for Res<'a, T>
where
    T: Resource,
{
    fn param_type() -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }
}

impl<'a, T> LocalSystemParam for ResMut<'a, T>
where
    T: Resource,
{
    fn param_type() -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }
}

impl<'a, T> LocalSystemParam for Option<Res<'a, T>>
where
    T: Resource,
{
    fn param_type() -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }
}

impl<'a, T> LocalSystemParam for Option<ResMut<'a, T>>
where
    T: Resource,
{
    fn param_type() -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }
}
