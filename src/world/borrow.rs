use crate::components::Component;
use crate::query::ComponentView;
use crate::resources::{Resource, ResourceCell, ResourceView};
use crate::storage::{ComponentStorage, Ticks};
use crate::utils::{panic_missing_comp, panic_missing_res};
use crate::world::World;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;

/// Shared view over a component storage.
pub type Comp<'a, T> = ComponentView<'a, T, AtomicRef<'a, ComponentStorage>>;
/// Exclusive view over a component storage.
pub type CompMut<'a, T> = ComponentView<'a, T, AtomicRefMut<'a, ComponentStorage>>;
/// Shared view over a resource.
pub type Res<'a, T> = ResourceView<T, AtomicRef<'a, ResourceCell>>;
/// Exclusive view over a resource.
pub type ResMut<'a, T> = ResourceView<T, AtomicRefMut<'a, ResourceCell>>;

/// Trait used to borrow component storages and resources from a `World`.
pub trait BorrowWorld<'a> {
    type Item;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item;
}

impl<'a, 'b, T> BorrowWorld<'a> for Comp<'b, T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        let (storage, info) = world
            .component_storages()
            .borrow_with_info(&TypeId::of::<T>())
            .unwrap_or_else(|| panic_missing_comp::<T>());

        unsafe { Comp::new(storage, info, world.tick(), change_tick) }
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for CompMut<'b, T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        let (storage, info) = world
            .component_storages()
            .borrow_with_info_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic_missing_comp::<T>());

        unsafe { CompMut::new(storage, info, world.tick(), change_tick) }
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Res<'b, T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        world
            .resource_storage()
            .borrow(&TypeId::of::<T>())
            .map(|cell| unsafe { Res::new(cell, world.tick(), change_tick) })
            .unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for ResMut<'b, T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        world
            .resource_storage()
            .borrow_mut(&TypeId::of::<T>())
            .map(|cell| unsafe { ResMut::new(cell, world.tick(), change_tick) })
            .unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<Res<'b, T>>
where
    T: Resource,
{
    type Item = Option<Res<'a, T>>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        world
            .resource_storage()
            .borrow(&TypeId::of::<T>())
            .map(|cell| unsafe { Res::new(cell, world.tick(), change_tick) })
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<ResMut<'b, T>>
where
    T: Resource,
{
    type Item = Option<ResMut<'a, T>>;

    fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
        world
            .resource_storage()
            .borrow_mut(&TypeId::of::<T>())
            .map(|cell| unsafe { ResMut::new(cell, world.tick(), change_tick) })
    }
}

macro_rules! impl_borrow_world {
	($($ty:ident),+) => {
		impl<'a, $($ty),+> BorrowWorld<'a> for ($($ty,)+)
		where
			$($ty: BorrowWorld<'a>,)+
		{
			type Item = ($($ty::Item,)+);

			fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
				($($ty::borrow(world, change_tick),)+)
			}
		}
	};
}

impl_borrow_world!(A);
impl_borrow_world!(A, B);
impl_borrow_world!(A, B, C);
impl_borrow_world!(A, B, C, D);
impl_borrow_world!(A, B, C, D, E);
impl_borrow_world!(A, B, C, D, E, F);
impl_borrow_world!(A, B, C, D, E, F, G);
impl_borrow_world!(A, B, C, D, E, F, G, H);
impl_borrow_world!(A, B, C, D, E, F, G, H, I);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_borrow_world!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
