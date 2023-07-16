use crate::resources::{Res, ResMut, Resource, Resources, SyncResources};
use crate::storage::Component;
use crate::systems::SystemDataDescriptor;
use crate::world::{Comp, CompMut, Entities, World};

/// Trait for borrowing system data from [`World`].
pub trait WorldSystemData: SystemDataDescriptor {
    /// Borrows the system data.
    fn borrow(world: &World) -> Self::SystemData<'_>;
}

/// Trait for borrowing system data from [`Resources`].
pub trait ResourcesSystemData: SystemDataDescriptor {
    /// Borrows the system data.
    fn borrow(resources: &Resources) -> Self::SystemData<'_>;
}

/// Trait for borrowing system data from [`SyncResources`].
pub trait SyncResourcesSystemData: ResourcesSystemData {
    /// Borrows the system data.
    fn borrow(resources: SyncResources<'_>) -> Self::SystemData<'_>;
}

/// Trait for borrowing system data from either [`World`] or [`Resources`].
pub trait LocalSystemData: SystemDataDescriptor {
    /// Borrows the system data.
    fn borrow<'a>(world: &'a World, resources: &'a Resources) -> Self::SystemData<'a>;
}

/// Trait for borrowing system data from either [`World`] or [`SyncResources`].
pub trait SystemData: LocalSystemData {
    /// Borrows the system data.
    fn borrow<'a>(world: &'a World, resources: SyncResources<'a>) -> Self::SystemData<'a>;
}

impl WorldSystemData for Entities<'_> {
    #[inline]
    fn borrow(world: &World) -> Self::SystemData<'_> {
        world.borrow_entities()
    }
}

impl LocalSystemData for Entities<'_> {
    #[inline]
    fn borrow<'a>(world: &'a World, _: &'a Resources) -> Self::SystemData<'a> {
        world.borrow_entities()
    }
}

impl SystemData for Entities<'_> {
    #[inline]
    fn borrow<'a>(world: &'a World, _: SyncResources<'a>) -> Self::SystemData<'a> {
        world.borrow_entities()
    }
}

macro_rules! impl_comp_system_data {
    ($Comp:ty, $borrow_method:ident) => {
        impl<T> WorldSystemData for $Comp
        where
            T: Component,
        {
            #[inline]
            fn borrow(world: &World) -> Self::SystemData<'_> {
                world.$borrow_method()
            }
        }

        impl<T> LocalSystemData for $Comp
        where
            T: Component,
        {
            #[inline]
            fn borrow<'a>(world: &'a World, _: &'a Resources) -> Self::SystemData<'a> {
                world.$borrow_method()
            }
        }

        impl<T> SystemData for $Comp
        where
            T: Component,
        {
            #[inline]
            fn borrow<'a>(world: &'a World, _: SyncResources<'a>) -> Self::SystemData<'a> {
                world.$borrow_method()
            }
        }
    };
}

macro_rules! impl_res_system_data {
    ($Res:ty, $SendSyncTrait:ident, $borrow_method:ident) => {
        impl<T> ResourcesSystemData for $Res
        where
            T: Resource,
        {
            #[inline]
            fn borrow(resources: &Resources) -> Self::SystemData<'_> {
                resources.$borrow_method()
            }
        }

        impl<T> SyncResourcesSystemData for $Res
        where
            T: Resource + $SendSyncTrait,
        {
            #[inline]
            fn borrow(resources: SyncResources<'_>) -> Self::SystemData<'_> {
                resources.$borrow_method()
            }
        }

        impl<T> LocalSystemData for $Res
        where
            T: Resource,
        {
            #[inline]
            fn borrow<'a>(_: &'a World, resources: &'a Resources) -> Self::SystemData<'a> {
                resources.$borrow_method()
            }
        }

        impl<T> SystemData for $Res
        where
            T: Resource + $SendSyncTrait,
        {
            #[inline]
            fn borrow<'a>(_: &'a World, resources: SyncResources<'a>) -> Self::SystemData<'a> {
                resources.$borrow_method()
            }
        }
    };
}

impl_comp_system_data!(Comp<'_, T>, borrow);
impl_comp_system_data!(CompMut<'_, T>, borrow_mut);
impl_res_system_data!(Res<'_, T>, Sync, borrow);
impl_res_system_data!(Option<Res<'_, T>>, Sync, try_borrow);
impl_res_system_data!(ResMut<'_, T>, Send, borrow_mut);
impl_res_system_data!(Option<ResMut<'_, T>>, Send, try_borrow_mut);
