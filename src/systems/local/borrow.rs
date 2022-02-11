use crate::resources::{Res, ResMut, Resource, Resources};
use crate::storage::Component;
use crate::utils::panic_missing_res;
use crate::world::{Comp, CompMut, Entities, World};

/// Trait implemented by local system parameters to borrow data.
pub trait BorrowLocalSystemData<'a> {
    /// System data to borrow.
    type Item: 'a;

    /// Borrows the system data.
    fn borrow(world: &'a World, resources: &'a Resources) -> Self::Item;
}

impl<'a, 'b> BorrowLocalSystemData<'a> for Entities<'b> {
    type Item = Entities<'a>;

    fn borrow(world: &'a World, _resources: &'a Resources) -> Self::Item {
        world.borrow_entities()
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for Comp<'b, T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn borrow(world: &'a World, _resources: &'a Resources) -> Self::Item {
        world.borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for CompMut<'b, T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn borrow(world: &'a World, _resources: &'a Resources) -> Self::Item {
        world.borrow_mut::<T>()
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for Res<'b, T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn borrow(_world: &'a World, resources: &'a Resources) -> Self::Item {
        resources.borrow::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for ResMut<'b, T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn borrow(_world: &'a World, resources: &'a Resources) -> Self::Item {
        resources.borrow_mut::<T>().unwrap_or_else(|| panic_missing_res::<T>())
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for Option<Res<'b, T>>
where
    T: Resource,
{
    type Item = Option<Res<'a, T>>;

    fn borrow(_world: &'a World, resources: &'a Resources) -> Self::Item {
        resources.borrow::<T>()
    }
}

impl<'a, 'b, T> BorrowLocalSystemData<'a> for Option<ResMut<'b, T>>
where
    T: Resource,
{
    type Item = Option<ResMut<'a, T>>;

    fn borrow(_world: &'a World, resources: &'a Resources) -> Self::Item {
        resources.borrow_mut::<T>()
    }
}
