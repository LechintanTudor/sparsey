use crate::registry::World;

pub trait BorrowWorld<'a> {
    fn borrow_world(world: &'a World) -> Self;
}

pub(crate) trait BorrowWorldUnsafe<'a> {
    unsafe fn borrow_world_unsafe(world: &'a World) -> Self;
}

macro_rules! impl_borrow_world {
    ($($b:ident),+) => {
        impl<'a, $($b,)+> BorrowWorld<'a> for ($($b,)+)
        where
            $($b: BorrowWorld<'a>,)+
        {
            fn borrow_world(world: &'a World) -> Self {
                ($(<$b as BorrowWorld<'a>>::borrow_world(world),)+)
            }
        }

        impl<'a, $($b,)+> BorrowWorldUnsafe<'a> for ($($b,)+)
        where
            $($b: BorrowWorldUnsafe<'a>,)+
        {
            unsafe fn borrow_world_unsafe(world: &'a World) -> Self {
                ($(<$b as BorrowWorldUnsafe<'a>>::borrow_world_unsafe(world),)+)
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
