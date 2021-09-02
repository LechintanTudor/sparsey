use crate::query::ComponentView;
use crate::storage::ComponentStorage;
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub type Comp<'a, T> = ComponentView<'a, T, AtomicRef<'a, ComponentStorage>>;
pub type CompMut<'a, T> = ComponentView<'a, T, AtomicRefMut<'a, ComponentStorage>>;
