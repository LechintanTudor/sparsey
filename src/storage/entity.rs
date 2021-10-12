use std::num::NonZeroU32;

/// Used by `EntityStorage` to recycle indexes. Entities with the same id and
/// different `Version`s are considered different.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Default for Version {
    fn default() -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(1)) }
    }
}

impl Version {
    /// Default `Version` of an `Entity`.
    pub const DEFAULT: Version = unsafe { Self::new(NonZeroU32::new_unchecked(1)) };

    /// Creates a new version with the given `id`.
    pub const fn new(id: NonZeroU32) -> Self {
        Self(id)
    }

    /// Returns the `id` of the version.
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
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    /// Creates a new entity with the given id and default `Version`.
    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            version: Version::DEFAULT,
        }
    }

    /// Returns the id of the entity.
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Returns the id of the entity, extended to a usize.
    pub const fn index(&self) -> usize {
        self.id as _
    }

    /// Returns the `Version` of the entity.
    pub const fn version(&self) -> Version {
        self.version
    }
}

/// Used internally by `SparseArray` to map `Entity` indexes to dense indexes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct IndexEntity {
    id: u32,
    version: Version,
}

impl IndexEntity {
    pub const fn new(id: u32, version: Version) -> Self {
        Self { id, version }
    }

    pub const fn index(&self) -> usize {
        self.id as _
    }

    pub const fn version(&self) -> Version {
        self.version
    }
}
