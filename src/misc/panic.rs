use std::any;

#[cold]
#[inline(never)]
pub fn panic_missing_comp<T>() -> ! {
	panic!(
		"Tried to access missing component storage `{}`",
		any::type_name::<T>()
	)
}

#[cold]
#[inline(never)]
pub fn panic_missing_res<T>() -> ! {
	panic!(
		"Tried to access missing resource `{}`",
		any::type_name::<T>()
	)
}
