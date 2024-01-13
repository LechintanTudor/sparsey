use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Version(pub NonZeroU32);

impl Version {
    pub const FIRST: Self = unsafe { Self(NonZeroU32::new_unchecked(1)) };

    #[inline]
    #[must_use]
    pub const fn new(index: u32) -> Option<Self> {
        match NonZeroU32::new(index) {
            Some(version) => Some(Self(version)),
            None => None,
        }
    }

    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        Self(NonZeroU32::new_unchecked(index))
    }

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

#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub index: u32,
    pub version: Version,
}

impl Entity {
    #[inline]
    #[must_use]
    pub const fn sparse(&self) -> usize {
        self.index as usize
    }
}

#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DenseEntity {
    pub index: u32,
    pub version: Version,
}

impl DenseEntity {
    #[inline]
    #[must_use]
    pub const fn dense(&self) -> usize {
        self.index as usize
    }
}

macro_rules! impl_entity_common {
    ($Entity:ident) => {
        impl $Entity {
            #[inline]
            #[must_use]
            pub const fn new(index: u32, version: Version) -> Self {
                Self { index, version }
            }

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
