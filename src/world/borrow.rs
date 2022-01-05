use crate::components::Component;
use crate::resources::Resource;
use crate::storage::ComponentStorage;
use crate::utils::{impl_generic_1_16, panic_missing_comp, panic_missing_res};
use crate::world::World;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;

/// Shared view over a component storage.
pub type Comp<'a, T> = ComponentView<'a, T, AtomicRef<'a, ComponentStorage>>;
/// Exclusive view over a component storage.
pub type CompMut<'a, T> = ComponentView<'a, T, AtomicRefMut<'a, ComponentStorage>>;
/// Shared view over a resource.
pub type Res<'a, T> = AtomicRef<'a, T>;
/// Exclusive view over a resource.
pub type ResMut<'a, T> = AtomicRefMut<'a, T>;

/// Trait used to borrow component storages and resources from a `World`.
pub trait BorrowWorld<'a> {
    type Item;

    fn borrow(world: &'a World) -> Self::Item;
}

impl<'a, 'b, T> BorrowWorld<'a> for Comp<'b, T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        let (storage, info) = world
            .component_storages()
            .borrow_with_info(&TypeId::of::<T>())
            .unwrap_or_else(|| panic_missing_comp::<T>());

        unsafe { Comp::new(storage, info) }
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for CompMut<'b, T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        let (storage, info) = world
            .component_storages()
            .borrow_with_info_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic_missing_comp::<T>());

        unsafe { CompMut::new(storage, info) }
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Res<'b, T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.resource_storage().borrow::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for ResMut<'b, T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.resource_storage().borrow_mut::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<Res<'b, T>>
where
    T: Resource,
{
    type Item = Option<Res<'a, T>>;

    fn borrow(world: &'a World) -> Self::Item {
        world.resource_storage().borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<ResMut<'b, T>>
where
    T: Resource,
{
    type Item = Option<ResMut<'a, T>>;

    fn borrow(world: &'a World) -> Self::Item {
        world.resource_storage().borrow_mut::<T>()
    }
}

macro_rules! impl_borrow_world {
    ($($ty:ident),+) => {
        impl<'a, $($ty),+> BorrowWorld<'a> for ($($ty,)+)
        where
            $($ty: BorrowWorld<'a>,)+
        {
            type Item = ($($ty::Item,)+);

            fn borrow(world: &'a World) -> Self::Item {
                ($($ty::borrow(world),)+)
            }
        }
    };
}

impl_generic_1_16!(impl_borrow_world);
