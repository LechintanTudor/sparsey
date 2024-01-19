use crate::entity::{Comp, CompMut, Component, ComponentData, Entities};
use crate::resource::{Res, ResMut, Resource};

/// The kind of data that can be borrowed from a registry.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SystemParamKind {
    /// View over all entities in an [`EntityStorage`](crate::entity::EntityStorage).
    Entities,
    /// Shared view over all components of a given type.
    Comp(ComponentData),
    /// Exclusive view over all components of a given type.
    CompMut(ComponentData),
    /// Shared view over a resource of a given type.
    Res(ComponentData),
    /// Exclusive view over a resource of a given type.
    ResMut(ComponentData),
}

impl SystemParamKind {
    /// Returns whether two system parameter kinds conflict, thus preventing two systems from
    /// running in parallel.
    #[inline]
    #[must_use]
    pub fn conflicts_with(self, other: Self) -> bool {
        #[allow(clippy::match_same_arms)]
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

/// Trait implemented by types that can be borrowed by systems during execution.
pub trait SystemParam {
    /// The kind of system parameter.
    const KIND: SystemParamKind;

    /// The system parameter generic over the lifetime `'a`.
    type Param<'a>;
}

impl SystemParam for Entities<'_> {
    const KIND: SystemParamKind = SystemParamKind::Entities;

    type Param<'a> = Entities<'a>;
}

impl<T> SystemParam for Comp<'_, T>
where
    T: Component,
{
    const KIND: SystemParamKind = SystemParamKind::Comp(ComponentData::new::<T>());

    type Param<'a> = Comp<'a, T>;
}

impl<T> SystemParam for CompMut<'_, T>
where
    T: Component,
{
    const KIND: SystemParamKind = SystemParamKind::CompMut(ComponentData::new::<T>());

    type Param<'a> = CompMut<'a, T>;
}

impl<T> SystemParam for Res<'_, T>
where
    T: Resource,
{
    const KIND: SystemParamKind = SystemParamKind::Res(ComponentData::new::<T>());

    type Param<'a> = Res<'a, T>;
}

impl<T> SystemParam for ResMut<'_, T>
where
    T: Resource,
{
    const KIND: SystemParamKind = SystemParamKind::ResMut(ComponentData::new::<T>());

    type Param<'a> = ResMut<'a, T>;
}

impl<T> SystemParam for Option<Res<'_, T>>
where
    T: Resource,
{
    const KIND: SystemParamKind = SystemParamKind::Res(ComponentData::new::<T>());

    type Param<'a> = Option<Res<'a, T>>;
}

impl<T> SystemParam for Option<ResMut<'_, T>>
where
    T: Resource,
{
    const KIND: SystemParamKind = SystemParamKind::ResMut(ComponentData::new::<T>());

    type Param<'a> = Option<ResMut<'a, T>>;
}
