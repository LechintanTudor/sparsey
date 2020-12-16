use crate::{
    entity::Entity,
    registry::{BorrowFromWorld, RawViewMut},
};
use std::any::TypeId;

pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

pub trait ComponentSource<'a>
where
    Self: Sized,
{
    type Target: BorrowFromWorld<'a>;
    type Components: AsRef<[TypeId]>;

    fn components() -> Self::Components;

    fn insert(target: &mut Self::Target, entity: Entity, components: Self);

    fn remove(target: &mut Self::Target, entity: Entity) -> Option<Self>;

    fn delete(target: &mut Self::Target, entity: Entity);
}

macro_rules! impl_component_source {
    ($len:tt, $(($ty:ident, $idx:tt)),+) => {
        impl<'a, $($ty,)+> ComponentSource<'a> for ($($ty,)+)
        where
            $($ty: Component,)+
        {
            type Target = ($(RawViewMut<'a, $ty>,)+);
            type Components = [TypeId; $len];

            fn components() -> Self::Components {
                [$(TypeId::of::<$ty>(),)+]
            }

            fn insert(target: &mut Self::Target, entity: Entity, components: Self) {
                $(target.$idx.set.insert(entity, components.$idx);)+
            }

            fn remove(target: &mut Self::Target, entity: Entity) -> Option<Self> {
                let components = (
                    $(target.$idx.set.remove(entity),)+
                );

                Some((
                    $(components.$idx?,)+
                ))
            }

            fn delete(target: &mut Self::Target, entity: Entity) {
                $(target.$idx.set.remove(entity);)+
            }
        }
    };
}

impl_component_source!(1, (A, 0));
impl_component_source!(2, (A, 0), (B, 1));
impl_component_source!(3, (A, 0), (B, 1), (C, 2));
impl_component_source!(4, (A, 0), (B, 1), (C, 2), (D, 3));
impl_component_source!(5, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_component_source!(6, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_component_source!(7, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
impl_component_source!(
    8,
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7)
);
