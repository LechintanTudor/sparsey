use crate::{
    registry::{Component, World},
    storage::SparseSet,
};
use atomic_refcell::AtomicRefMut;

pub trait BorrowFromWorld<'a> {
    fn borrow(world: &'a World) -> Self;
}

pub struct RawViewMut<'a, T> {
    pub set: AtomicRefMut<'a, SparseSet<T>>,
}

impl<'a, T> BorrowFromWorld<'a> for RawViewMut<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        Self {
            set: world.borrow_raw_mut().unwrap(),
        }
    }
}

macro_rules! impl_borrow_from_world {
    ($($b:ident),+) => {
        impl<'a, $($b,)+> BorrowFromWorld<'a> for ($($b,)+)
        where
            $($b: BorrowFromWorld<'a>,)+
        {
            fn borrow(world: &'a World) -> Self {
                ($(<$b as BorrowFromWorld<'a>>::borrow(world),)+)
            }
        }
    };
}

impl_borrow_from_world!(A);
impl_borrow_from_world!(A, B);
impl_borrow_from_world!(A, B, C);
impl_borrow_from_world!(A, B, C, D);
impl_borrow_from_world!(A, B, C, D, E);
impl_borrow_from_world!(A, B, C, D, E, F);
impl_borrow_from_world!(A, B, C, D, E, F, G);
impl_borrow_from_world!(A, B, C, D, E, F, G, H);
impl_borrow_from_world!(A, B, C, D, E, F, G, H, I);
impl_borrow_from_world!(A, B, C, D, E, F, G, H, I, J);
impl_borrow_from_world!(A, B, C, D, E, F, G, H, I, J, K);
impl_borrow_from_world!(A, B, C, D, E, F, G, H, I, J, K, L);
