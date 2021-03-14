use std::num::NonZeroU32;

/// Unique identifier for a set of components in the `World`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Entity {
    id: u32,
    ver: Version,
}

impl Entity {
    /// Create a new `Entity` with the given id and version.
    pub const fn new(id: u32, ver: Version) -> Self {
        Self { id, ver }
    }

    /// Create a new `Entity` with the given id and the default version.
    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            ver: Version::FIRST,
        }
    }

    /// Get the id of the `Entity`.
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Get the version of the `Entity`.
    pub const fn ver(&self) -> Version {
        self.ver
    }

    /// Get the index in the `SparseVec` for the `Entity`.
    pub const fn index(&self) -> usize {
        self.id as _
    }
}

/// Maps `SparseVec` indexes to tightly packed array indexes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct IndexEntity {
    id: u32,
    ver: Version,
}

impl IndexEntity {
    /// Create a new `IndexEntity` with the given id and version.
    pub const fn new(id: u32, ver: Version) -> Self {
        Self { id, ver }
    }

    /// Create a new `IndexEntity` with the given id and the default version.
    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            ver: Version::FIRST,
        }
    }

    /// Get the id of the `IndexEntity`.
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Get the version of the `IndexEntity`.
    pub const fn ver(&self) -> Version {
        self.ver
    }

    /// Get the index in the `SparseVec` for the `IndexEntity`.
    pub const fn index(&self) -> usize {
        self.id as _
    }
}

/// Number representing the version of an `Entity`.
/// Enables reusing `Entity` ids.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Version {
    /// First available version.
    pub const FIRST: Self = unsafe { Self::new_unchecked(1) };

    /// Create a new `Version` with the given `id`.
    /// Panics if the given `id` is zero.
    pub fn new(id: u32) -> Self {
        Self(NonZeroU32::new(id).unwrap())
    }

    /// Create a new `Version` with the given `id`.
    /// It is undefined behavior if the `id` is zero.
    pub const unsafe fn new_unchecked(id: u32) -> Self {
        Self(NonZeroU32::new_unchecked(id))
    }

    /// Get the id of the `Version`.
    pub const fn id(&self) -> u32 {
        self.0.get()
    }

    /// Get the next available `Version`, if any.
    pub const fn next(&self) -> Option<Self> {
        if self.id() != u32::MAX {
            Some(Self(unsafe { NonZeroU32::new_unchecked(self.id() + 1) }))
        } else {
            None
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::FIRST
    }
}
