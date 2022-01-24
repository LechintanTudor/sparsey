use crate::resources::Resource;
use crate::storage::Component;
use crate::utils::{panic_missing_comp, panic_missing_res};
use crate::world::{Comp, CompMut, Entities, Res, ResMut, SyncWorld, World};

/// Trait used to borrow component storages and resources from a `World`.
pub trait BorrowWorld<'a> {
    type Item: 'a;

    fn borrow(world: &'a World) -> Self::Item;
}

impl<'a, 'b> BorrowWorld<'a> for Entities<'b> {
    type Item = Entities<'a>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_entities()
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Comp<'b, T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_comp().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for CompMut<'b, T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_comp_mut().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Res<'b, T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_res().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for ResMut<'b, T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_res_mut().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<Res<'b, T>>
where
    T: Resource,
{
    type Item = Option<Res<'a, T>>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_res()
    }
}

impl<'a, 'b, T> BorrowWorld<'a> for Option<ResMut<'b, T>>
where
    T: Resource,
{
    type Item = Option<ResMut<'a, T>>;

    fn borrow(world: &'a World) -> Self::Item {
        world.borrow_res_mut()
    }
}

pub trait BorrowSyncWorld<'a>: BorrowWorld<'a> {
    fn borrow(world: &'a SyncWorld) -> Self::Item;
}

impl<'a, 'b> BorrowSyncWorld<'a> for Entities<'b> {
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_entities()
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for Comp<'b, T>
where
    T: Component,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_comp().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for CompMut<'b, T>
where
    T: Component,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_comp_mut().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for Res<'b, T>
where
    T: Resource + Sync,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_res().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for ResMut<'b, T>
where
    T: Resource + Send,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_res_mut().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for Option<Res<'b, T>>
where
    T: Resource + Sync,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_res()
    }
}

impl<'a, 'b, T> BorrowSyncWorld<'a> for Option<ResMut<'b, T>>
where
    T: Resource + Send,
{
    fn borrow(world: &'a SyncWorld) -> Self::Item {
        world.borrow_res_mut()
    }
}

macro_rules! impl_tuple_borrows {
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

        impl<'a, $($ty),+> BorrowSyncWorld<'a> for ($($ty,)+)
        where
            $($ty: BorrowSyncWorld<'a>,)+
        {
            fn borrow(world: &'a SyncWorld) -> Self::Item {
                ($(<$ty as BorrowSyncWorld>::borrow(world),)+)
            }
        }
    };
}

impl_tuple_borrows!(A, B);
impl_tuple_borrows!(A, B, C);
impl_tuple_borrows!(A, B, C, D);
impl_tuple_borrows!(A, B, C, D, E);
impl_tuple_borrows!(A, B, C, D, E, F);
impl_tuple_borrows!(A, B, C, D, E, F, G);
impl_tuple_borrows!(A, B, C, D, E, F, G, H);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple_borrows!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
