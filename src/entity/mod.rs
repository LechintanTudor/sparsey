//! Entity storage and allocation.

mod entity_allocator;
mod entity_sparse_set;
mod entity_storage;
mod sparse_vec;

pub use self::sparse_vec::*;

pub(crate) use self::entity_allocator::*;
pub(crate) use self::entity_sparse_set::*;
pub(crate) use self::entity_storage::*;

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU32;

/// Version used to distinguish between entities with the same index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Version(pub NonZeroU32);

impl Version {
    /// The first valid version.
    pub const FIRST: Self = Self(NonZeroU32::MIN);

    /// The last valid version.
    pub const LAST: Self = Self(NonZeroU32::MAX);

    /// Returns the next available version, if any.
    #[inline]
    #[must_use]
    pub const fn next(&self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(version) => Some(Self(version)),
            None => None,
        }
    }
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self::FIRST
    }
}

/// Uniquely identifies a set of components in a
/// [`World`](crate::world::World).
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// The sparse index of the entity.
    pub index: u32,
    /// The version of the entity.
    pub version: Version,
}

impl Entity {
    /// Returns the index of the entity extended to a [`usize`].
    #[inline]
    #[must_use]
    pub const fn sparse(&self) -> usize {
        self.index as usize
    }
}

/// Used internally by [`SparseVec`] to map sparse indexes to dense indexes.
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DenseEntity {
    /// The dense index of the entity.
    pub index: u32,
    /// The version of the entity.
    pub version: Version,
}

impl DenseEntity {
    /// Returns the index of the entity extended to a [`usize`].
    #[inline]
    #[must_use]
    pub const fn dense(&self) -> usize {
        self.index as usize
    }
}

macro_rules! impl_entity_common {
    ($Entity:ident) => {
        impl $Entity {
            /// Creates a new entity with the given `index` and `version`.
            #[inline]
            #[must_use]
            pub const fn new(index: u32, version: Version) -> Self {
                Self { index, version }
            }

            /// Creates a new entity with the given `index` and default
            /// version.
            #[inline]
            #[must_use]
            pub const fn with_index(index: u32) -> Self {
                Self {
                    index,
                    version: Version::FIRST,
                }
            }
        }

        impl PartialOrd for $Entity {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $Entity {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.version
                    .cmp(&other.version)
                    .then(self.index.cmp(&other.index))
            }
        }

        impl fmt::Debug for $Entity {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($Entity))
                    .field("index", &self.index)
                    .field("version", &self.version.0)
                    .finish()
            }
        }
    };
}

impl_entity_common!(Entity);
impl_entity_common!(DenseEntity);
