use crate::{
    entity::Entity,
    registry::{BorrowFromWorld, RawViewMut},
};

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

    fn insert(target: &mut Self::Target, entity: Entity, components: Self);

    fn remove(target: &mut Self::Target, entity: Entity) -> Option<Self>;

    fn delete(target: &mut Self::Target, entity: Entity);
}

macro_rules! impl_component_source {
    ($(($ty:ident, $idx:tt)),+) => {
        impl<'a, $($ty,)+> ComponentSource<'a> for ($($ty,)+)
        where
            $($ty: Component,)+
        {
            type Target = ($(RawViewMut<'a, $ty>,)+);

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

impl_component_source!((A, 0));
impl_component_source!((A, 0), (B, 1));
impl_component_source!((A, 0), (B, 1), (C, 2));
impl_component_source!((A, 0), (B, 1), (C, 2), (D, 3));
