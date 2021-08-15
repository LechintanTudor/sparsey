use std::num::NonZeroU32;

/// Handle used to fetch components from `ComponentStorages`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Entity {
	index: u32,
	version: Version,
}

impl Entity {
	/// Creates a new entity with the given `index` and `version`.
	pub const fn new(index: u32, version: Version) -> Self {
		Self { index, version }
	}

	/// Creates a new entity with the given `index` and default version.
	pub const fn with_index(index: u32) -> Self {
		unsafe {
			Self {
				index,
				version: Version::new_unchecked(1),
			}
		}
	}

	/// Returns the `index` of the entity, extended to a usize.
	pub const fn index(&self) -> usize {
		self.index as _
	}

	/// Returns the `version` of the entity.
	pub const fn version(&self) -> Version {
		self.version
	}

	/// Creates an entity with the same index as the current one and next
	/// version if the version is not the last possible one.
	pub const fn with_next_version(&self) -> Option<Self> {
		if self.version.id() < u32::MAX {
			unsafe {
				Some(Self::new(
					self.index,
					Version::new_unchecked(self.version.id() + 1),
				))
			}
		} else {
			None
		}
	}
}

/// Used internally by `SparseArray` to map `Entity` indexes to dense indexes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct IndexEntity {
	index: u32,
	version: Version,
}

impl IndexEntity {
	/// Creates a new index entity with the given `index` and `version`.
	pub const fn new(index: u32, version: Version) -> Self {
		Self { index, version }
	}

	/// Creates a new index entity with the given `index` and default version.
	pub const fn with_index(index: u32) -> Self {
		unsafe {
			Self {
				index,
				version: Version::new_unchecked(1),
			}
		}
	}

	/// Returns the `index` of the index entity, extended to a usize.
	pub const fn index(&self) -> usize {
		self.index as _
	}

	/// Returns the `version` of the index entity.
	pub const fn version(&self) -> Version {
		self.version
	}

	/// Creates an index entity with the same index as the current one and next
	/// version if the version is not the last possible one.
	pub const fn with_next_version(&self) -> Option<Self> {
		if self.version.id() < u32::MAX {
			unsafe {
				Some(Self::new(
					self.index,
					Version::new_unchecked(self.version.id() + 1),
				))
			}
		} else {
			None
		}
	}
}

/// Used by `Entities` to recycle indexes. `Entities` with the same index and
/// different `Versions` are considered different.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Default for Version {
	fn default() -> Self {
		unsafe { Self(NonZeroU32::new_unchecked(1)) }
	}
}

impl Version {
	/// Creates a new version with the given `id`.
	pub fn new(id: u32) -> Self {
		Self(NonZeroU32::new(id).expect("Version ID cannot be zero"))
	}

	/// Creates a new version with the given `id`. The `id` must be `non-zero`.
	pub const unsafe fn new_unchecked(id: u32) -> Self {
		Self(NonZeroU32::new_unchecked(id))
	}

	/// Returns the `id` of the version.
	pub const fn id(&self) -> u32 {
		self.0.get()
	}
}
