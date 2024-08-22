//! Manages entities and their associated components.

mod entity_allocator;
mod entity_sparse_set;
mod sparse_vec;

pub use self::sparse_vec::*;

pub(crate) use self::entity_allocator::*;
pub(crate) use self::entity_sparse_set::*;

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU32;

/// Version used for recylcling entity indexes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Version(pub NonZeroU32);

impl Version {
    /// The first valid version.
    pub const FIRST: Self = Self(NonZeroU32::MIN);

    /// The last valid version.
    pub const LAST: Self = Self(NonZeroU32::MAX);

    /// Creates a new version. Returns [`None`] if the `index` is zero.
    #[inline]
    #[must_use]
    pub const fn new(index: u32) -> Option<Self> {
        match NonZeroU32::new(index) {
            Some(version) => Some(Self(version)),
            None => None,
        }
    }

    /// Creates a new version without checking if the `index` is non-zero.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        Self(NonZeroU32::new_unchecked(index))
    }

    /// Returns the next valid version, if any.
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

/// Uniquely identifies a set of components in an [`EntityStorage`](crate::entity::EntityStorage).
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Sparse index for accessing [`SparseVec`](crate::entity::SparseVec).
    pub index: u32,
    /// Version used for recycling entity indexes.
    pub version: Version,
}

impl Entity {
    /// Returns [`index`](Entity::index) extended to a [`usize`].
    #[inline]
    #[must_use]
    pub const fn sparse(&self) -> usize {
        self.index as usize
    }
}

/// Versioned index stored in [`SparseVec`](crate::entity::SparseVec).
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DenseEntity {
    /// Dense index used for accessing packed data in sparse sets.
    pub index: u32,
    /// Version used for recycling entity indexes.
    pub version: Version,
}

impl DenseEntity {
    /// Returns [`index`](Self::index) extended to a [`usize`].
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
            /// [`Version`](crate::entity::Version).
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
