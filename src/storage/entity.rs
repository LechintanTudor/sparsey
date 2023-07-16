use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::num::NonZeroU32;

/// Type used to tell apart entities with the same index. Entities with the same index and
/// different versions are considered different.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Version {
    /// Default version of an [`Entity`].
    pub const DEFAULT: Version = unsafe { Self(NonZeroU32::new_unchecked(1)) };

    /// Creates a new version with the given `indexd`.
    #[inline]
    pub const fn new(index: NonZeroU32) -> Self {
        Self(index)
    }

    /// Returns the index of the version.
    #[inline]
    #[must_use]
    pub const fn index(&self) -> u32 {
        self.0.get()
    }

    /// Returns the version after the current one, if any.
    #[inline]
    #[must_use]
    pub fn next(&self) -> Option<Version> {
        self.0.checked_add(1).map(Self)
    }
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Uniquely identifies a set of components in a [`World`](crate::world::World).
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Entity {
    index: u32,
    version: Version,
}

impl Entity {
    /// Returns the index of the entity extended to a [`usize`].
    #[inline]
    #[must_use]
    pub const fn sparse(&self) -> usize {
        self.index as _
    }
}

/// Used internally by `SparseArray` to store versioned dense indexes.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DenseEntity {
    index: u32,
    version: Version,
}

impl DenseEntity {
    /// Returns the index of the entity extended to a [`usize`].
    #[inline]
    #[must_use]
    pub const fn dense(&self) -> usize {
        self.index as _
    }
}

macro_rules! impl_entity_common {
    ($Entity:ident) => {
        impl $Entity {
            /// Creates a new entity with the given `index` and `version`.
            #[inline]
            pub const fn new(index: u32, version: Version) -> Self {
                Self { index, version }
            }

            /// Creates a new entity with the given `index` and default `version`.
            #[inline]
            pub const fn with_index(index: u32) -> Self {
                Self {
                    index,
                    version: Version::DEFAULT,
                }
            }

            /// Returns the index of the entity.
            #[inline]
            #[must_use]
            pub const fn index(&self) -> u32 {
                self.index
            }

            /// Returns the version of the entity.
            #[inline]
            #[must_use]
            pub const fn version(&self) -> Version {
                self.version
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
