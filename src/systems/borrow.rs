use crate::resources::{Res, ResMut, Resource, SyncResources};
use crate::storage::Component;
use crate::utils::{panic_missing_comp, panic_missing_res};
use crate::world::{Comp, CompMut, Entities, World};

pub trait BorrowSystemData<'a> {
    type Item;

    fn borrow(world: &'a World, resources: SyncResources<'a>) -> Self::Item;
}

impl<'a, 'b> BorrowSystemData<'a> for Entities<'b> {
    type Item = Entities<'a>;

    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow_entities()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Comp<'b, T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow::<T>().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for CompMut<'b, T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn borrow(world: &'a World, _resources: SyncResources<'a>) -> Self::Item {
        world.borrow_mut::<T>().unwrap_or_else(|| panic_missing_comp::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Res<'b, T>
where
    T: Resource + Sync,
{
    type Item = Res<'a, T>;

    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for ResMut<'b, T>
where
    T: Resource + Send,
{
    type Item = ResMut<'a, T>;

    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow_mut::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Option<Res<'b, T>>
where
    T: Resource + Sync,
{
    type Item = Option<Res<'a, T>>;

    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowSystemData<'a> for Option<ResMut<'b, T>>
where
    T: Resource + Send,
{
    type Item = Option<ResMut<'a, T>>;

    fn borrow(_world: &'a World, resources: SyncResources<'a>) -> Self::Item {
        resources.borrow_mut::<T>()
    }
}
