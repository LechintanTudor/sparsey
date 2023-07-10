use crate::resources::Resource;
use crate::storage::{Component, ComponentStorage};
use std::any::{self, TypeId};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Stores information about a component type.
pub struct ComponentData(Box<dyn AbstractComponentData>);

impl ComponentData {
    /// Creates a new `ComponentData` for components of type `T`.
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self(Box::<TypeWrapper<T>>::default())
    }

    #[inline]
    pub(crate) fn create_storage(&self) -> ComponentStorage {
        self.0.create_storage()
    }
}

/// Stores information about a resource type.
pub struct ResourceData(Box<dyn AbstractResourceData>);

impl ResourceData {
    /// Creates a new `ResourceData` for resources of type `T`.
    pub fn new<T>() -> Self
    where
        T: Resource,
    {
        Self(Box::<TypeWrapper<T>>::default())
    }
}

macro_rules! impl_type_data_common {
    ($TypeData:ident) => {
        impl $TypeData {
            /// Returns the [`TypeId`] of the wrapped type.
            #[inline]
            #[must_use]
            pub fn type_id(&self) -> TypeId {
                self.0.type_id()
            }

            /// Returns the type name of the wrapped type.
            #[inline]
            #[must_use]
            pub fn type_name(&self) -> &'static str {
                self.0.type_name()
            }
        }

        impl Clone for $TypeData {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl PartialEq for $TypeData {
            #[inline]
            fn eq(&self, other: &$TypeData) -> bool {
                self.type_id() == other.type_id()
            }
        }

        impl Eq for $TypeData {
            // Empty
        }

        impl PartialOrd for $TypeData {
            #[inline]
            fn partial_cmp(&self, other: &$TypeData) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $TypeData {
            #[inline]
            fn cmp(&self, other: &$TypeData) -> Ordering {
                self.type_id().cmp(&other.type_id())
            }
        }

        impl Hash for $TypeData {
            fn hash<H>(&self, state: &mut H)
            where
                H: Hasher,
            {
                self.type_id().hash(state);
            }
        }

        impl fmt::Debug for $TypeData {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($TypeData))
                    .field("type_id", &self.0.type_id())
                    .field("type_name", &self.0.type_name())
                    .finish()
            }
        }
    };
}

impl_type_data_common!(ComponentData);
impl_type_data_common!(ResourceData);

struct TypeWrapper<T>(PhantomData<*const T>);

unsafe impl<T> Send for TypeWrapper<T> {}
unsafe impl<T> Sync for TypeWrapper<T> {}

impl<T> Default for TypeWrapper<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

trait AbstractComponentData: Send + Sync + 'static {
    fn type_id(&self) -> TypeId;

    fn type_name(&self) -> &'static str;

    fn create_storage(&self) -> ComponentStorage;

    fn clone(&self) -> Box<dyn AbstractComponentData>;
}

impl<T> AbstractComponentData for TypeWrapper<T>
where
    T: Component,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> &'static str {
        any::type_name::<T>()
    }

    fn create_storage(&self) -> ComponentStorage {
        ComponentStorage::new::<T>()
    }

    fn clone(&self) -> Box<dyn AbstractComponentData> {
        Box::<Self>::default()
    }
}

trait AbstractResourceData: Send + Sync + 'static {
    fn type_id(&self) -> TypeId;

    fn type_name(&self) -> &'static str;

    fn clone(&self) -> Box<dyn AbstractResourceData>;
}

impl<T> AbstractResourceData for TypeWrapper<T>
where
    T: Resource,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> &'static str {
        any::type_name::<T>()
    }

    fn clone(&self) -> Box<dyn AbstractResourceData> {
        Box::<Self>::default()
    }
}
