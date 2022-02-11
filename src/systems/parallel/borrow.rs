use crate::resources::{Res, ResMut, Resource, SyncResources};
use crate::storage::Component;
use crate::systems::BorrowLocalSystemData;
use crate::utils::panic_missing_res;
use crate::world::{Comp, CompMut, Entities, World};

/// Trait implemented by system parameters to borrow data.
pub trait BorrowSystemData<'a>: BorrowLocalSystemData<'a> {
    /// Borrows the system data.
    fn borrow(world: &'a World, resources: SyncResources<'a>) -> Self::Item;
}

impl<'a, 'b> BorrowSystemData<'a> for Entities<'b> {
    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow_entities()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Comp<'b, T>
where
    T: Component,
{
    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for CompMut<'b, T>
where
    T: Component,
{
    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow_mut::<T>()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Res<'b, T>
where
    T: Resource + Sync,
{
    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for ResMut<'b, T>
where
    T: Resource + Send,
{
    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow_mut::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Option<Res<'b, T>>
where
    T: Resource + Sync,
{
    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Option<ResMut<'b, T>>
where
    T: Resource + Send,
{
    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow_mut::<T>()
    }
}
