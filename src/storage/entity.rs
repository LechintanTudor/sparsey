use std::fmt;
use std::num::NonZeroU32;

/// Type used to tell apart entities with the same id. Entities with the same id and different
/// `Version`s are considered different.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Default for Version {
    #[inline]
    fn default() -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(1)) }
    }
}

impl Version {
    /// Default `Version` of an `Entity`.
    pub const DEFAULT: Version = unsafe { Self::new(NonZeroU32::new_unchecked(1)) };

    /// Creates a new version with the given `id`.
    #[inline]
    pub const fn new(id: NonZeroU32) -> Self {
        Self(id)
    }

    /// Returns the `id` of the version.
    #[inline]
    pub const fn id(&self) -> u32 {
        self.0.get()
    }
}

/// Uniquely identifies a set of components in a `World`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Entity {
    id: u32,
    version: Version,
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity").field("id", &self.id).field("version", &self.version.0).finish()
    }
}

impl Entity {
    /// Creates a new entity with the given `id` and `Version`.
    #[inline]
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    /// Creates a new entity with the given `id` and default `Version`.
    #[inline]
    pub const fn with_id(id: u32) -> Self {
        Self { id, version: Version::DEFAULT }
    }

    /// Returns the id of the entity.
    #[inline]
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Returns the id of the entity extended to a usize.
    #[inline]
    pub const fn sparse(&self) -> usize {
        self.id as _
    }

    /// Returns the `Version` of the entity.
    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }
}

/// Used internally by `SparseArray` to map `Entity` indexes to dense
/// indexes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct IndexEntity {
    id: u32,
    version: Version,
}

impl fmt::Debug for IndexEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexEntity")
            .field("id", &self.id)
            .field("version", &self.version.0)
            .finish()
    }
}

impl IndexEntity {
    /// Creates a new `IndexEntity` with the given `id` and `Version`.
    #[inline]
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    /// Returns the `id` of the entity extended to a `usize`..
    #[inline]
    pub const fn dense(&self) -> usize {
        self.id as _
    }

    /// Returns the `Version` of the entity.
    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }
}
