use std::num::NonZeroU32;

/// Used by `EntityStorage` to recycle indexes. Entities with the same id and
/// different `Version`s are considered different.
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

/// Handle used to fetch components from `ComponentStorages`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Entity {
    id: u32,
    version: Version,
}

impl Entity {
    /// Creates a new entity with the given id`and `Version`.
    #[inline]
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    /// Creates a new entity with the given id and default `Version`.
    #[inline]
    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            version: Version::DEFAULT,
        }
    }

    /// Returns the id of the entity.
    #[inline]
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Returns the id of the entity, extended to a usize.
    #[inline]
    pub const fn index(&self) -> usize {
        self.id as _
    }

    /// Returns the `Version` of the entity.
    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }
}

/// Used internally by `EntitySparseArray` to map `Entity` indexes to dense
/// indexes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct IndexEntity {
    id: u32,
    version: Version,
}

impl IndexEntity {
    #[inline]
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    #[inline]
    pub const fn index(&self) -> usize {
        self.id as _
    }

    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }
}
