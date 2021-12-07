use std::any;

#[cold]
#[inline(never)]
pub(crate) fn panic_missing_comp<T>() -> ! {
    panic!("Tried to access missing component storage of type `{}`", any::type_name::<T>())
}

#[cold]
#[inline(never)]
pub(crate) fn panic_missing_res<T>() -> ! {
    panic!("Tried to access missing resource of type `{}`", any::type_name::<T>())
}
