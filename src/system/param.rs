use crate::entity::{Comp, CompMut, Component, ComponentData, Entities};
use crate::resource::{Res, ResMut, Resource};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SystemParamKind {
    Entities,
    Comp(ComponentData),
    CompMut(ComponentData),
    Res(ComponentData),
    ResMut(ComponentData),
}

impl SystemParamKind {
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

pub trait SystemParam {
    const KIND: SystemParamKind;

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
