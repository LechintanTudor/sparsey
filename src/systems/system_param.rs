use crate::layout::ComponentInfo;
use crate::resources::Resource;
use crate::storage::Component;
use crate::world::{BorrowSyncWorld, Comp, CompMut, Entities, Res, ResMut};
use std::any::TypeId;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SystemParamType {
    Entities,
    Comp(ComponentInfo),
    CompMut(ComponentInfo),
    Res(TypeId),
    ResMut(TypeId),
}

pub trait SystemParam: for<'a> BorrowSyncWorld<'a> {
    fn param_type(&self) -> SystemParamType;
}

impl<'a> SystemParam for Entities<'a> {
    fn param_type(&self) -> SystemParamType {
        SystemParamType::Entities
    }
}

impl<'a, T> SystemParam for Comp<'a, T>
where
    T: Component,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::Comp(ComponentInfo::new::<T>())
    }
}

impl<'a, T> SystemParam for CompMut<'a, T>
where
    T: Component,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::CompMut(ComponentInfo::new::<T>())
    }
}

impl<'a, T> SystemParam for Res<'a, T>
where
    T: Resource + Sync,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }
}

impl<'a, T> SystemParam for ResMut<'a, T>
where
    T: Resource + Send,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }
}

impl<'a, T> SystemParam for Option<Res<'a, T>>
where
    T: Resource + Sync,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::Res(TypeId::of::<T>())
    }
}

impl<'a, T> SystemParam for Option<ResMut<'a, T>>
where
    T: Resource + Send,
{
    fn param_type(&self) -> SystemParamType {
        SystemParamType::ResMut(TypeId::of::<T>())
    }
}
