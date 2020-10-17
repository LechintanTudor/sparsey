use crate::Component;

pub trait IterView {}

impl<T> IterView for &T where T: Component {}

impl<T> IterView for &mut T where T: Component {}

impl<T> IterView for Option<&T> where T: Component {}

impl<T> IterView for Option<&mut T> where T: Component {}

pub struct Iterator2 {}

// world.iter
