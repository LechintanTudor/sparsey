use std::hint::unreachable_unchecked;

pub(crate) trait UnsafeUnwrap {
    type Output;

    unsafe fn unsafe_unwrap(self) -> Self::Output;
}

impl<T> UnsafeUnwrap for Option<T> {
    type Output = T;

    unsafe fn unsafe_unwrap(self) -> Self::Output {
        debug_assert!(self.is_some());

        match self {
            Some(output) => output,
            None => unreachable_unchecked(),
        }
    }
}

impl<T, E> UnsafeUnwrap for Result<T, E> {
    type Output = T;

    unsafe fn unsafe_unwrap(self) -> Self::Output {
        debug_assert!(self.is_ok());

        match self {
            Ok(output) => output,
            Err(_) => unreachable_unchecked(),
        }
    }
}
