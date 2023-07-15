use crate::resources::{Res, ResMut, Resource};
use crate::storage::Component;
use crate::systems::SystemDataType;
use crate::utils::{ComponentData, ResourceData};
use crate::world::{Comp, CompMut, Entities};

pub trait SystemDataDescriptor {
    type SystemData<'a>;

    #[must_use]
    fn system_data_type() -> SystemDataType;
}

impl SystemDataDescriptor for Entities<'_> {
    type SystemData<'a> = Entities<'a>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::Entities
    }
}

impl<T> SystemDataDescriptor for Comp<'_, T>
where
    T: Component,
{
    type SystemData<'a> = Comp<'a, T>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::Comp(ComponentData::new::<T>())
    }
}

impl<T> SystemDataDescriptor for CompMut<'_, T>
where
    T: Component,
{
    type SystemData<'a> = CompMut<'a, T>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::CompMut(ComponentData::new::<T>())
    }
}

impl<T> SystemDataDescriptor for Res<'_, T>
where
    T: Resource,
{
    type SystemData<'a> = Res<'a, T>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::Res(ResourceData::new::<T>())
    }
}

impl<T> SystemDataDescriptor for ResMut<'_, T>
where
    T: Resource,
{
    type SystemData<'a> = ResMut<'a, T>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::ResMut(ResourceData::new::<T>())
    }
}

impl<T> SystemDataDescriptor for Option<Res<'_, T>>
where
    T: Resource,
{
    type SystemData<'a> = Option<Res<'a, T>>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::Res(ResourceData::new::<T>())
    }
}

impl<T> SystemDataDescriptor for Option<ResMut<'_, T>>
where
    T: Resource,
{
    type SystemData<'a> = Option<ResMut<'a, T>>;

    #[inline]
    fn system_data_type() -> SystemDataType {
        SystemDataType::ResMut(ResourceData::new::<T>())
    }
}
