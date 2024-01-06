use crate::entity::{Comp, CompMut, Component, ComponentSparseSet};
use atomic_refcell::AtomicRefCell;
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::{any, fmt};

#[derive(Default)]
pub struct ComponentStorage {
    components: Vec<AtomicRefCell<ComponentSparseSet>>,
    metadata: FxHashMap<TypeId, ComponentMetadata>,
}

impl ComponentStorage {
    pub fn register<T>(&mut self) -> bool
    where
        T: Component,
    {
        let Entry::Vacant(entry) = self.metadata.entry(TypeId::of::<T>()) else {
            return false;
        };

        entry.insert(ComponentMetadata {
            index: self.components.len(),
        });

        self.components
            .push(AtomicRefCell::new(ComponentSparseSet::new::<T>()));

        true
    }

    #[must_use]
    pub fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.metadata.contains_key(&TypeId::of::<T>())
    }

    #[must_use]
    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe { Comp::<T>::new(self.components.get_unchecked(metadata.index).borrow()) }
    }

    #[must_use]
    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        let Some(metadata) = self.metadata.get(&TypeId::of::<T>()) else {
            panic_missing_comp::<T>();
        };

        unsafe { CompMut::<T>::new(self.components.get_unchecked(metadata.index).borrow_mut()) }
    }
}

impl fmt::Debug for ComponentStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ComponentStorage))
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug)]
struct ComponentMetadata {
    index: usize,
}

#[cold]
#[inline(never)]
fn panic_missing_comp<T>() -> ! {
    panic!("Component '{}' was not registered", any::type_name::<T>());
}
