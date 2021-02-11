use std::num::NonZeroU32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Entity {
    id: u32,
    gen: Generation,
}

impl Entity {
    pub const fn new(id: u32, gen: Generation) -> Self {
        Self { id, gen }
    }

    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            gen: Generation::FIRST,
        }
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub const fn gen(&self) -> Generation {
        self.gen
    }

    pub const fn index(&self) -> usize {
        self.id as _
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IndexEntity {
    id: u32,
    gen: Generation,
}

impl IndexEntity {
    pub const fn new(id: u32, gen: Generation) -> Self {
        Self { id, gen }
    }

    pub const fn with_id(id: u32) -> Self {
        Self {
            id,
            gen: Generation::FIRST,
        }
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub const fn gen(&self) -> Generation {
        self.gen
    }

    pub const fn index(&self) -> usize {
        self.id as _
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Generation(NonZeroU32);

impl Generation {
    pub const FIRST: Self = unsafe { Self::new_unchecked(1) };
    pub const LAST: Self = unsafe { Self::new_unchecked(u32::MAX) };

    pub fn new(id: u32) -> Self {
        Self(NonZeroU32::new(id).unwrap())
    }

    pub const unsafe fn new_unchecked(id: u32) -> Self {
        Self(NonZeroU32::new_unchecked(id))
    }

    pub const fn id(&self) -> u32 {
        self.0.get()
    }

    pub const fn next(&self) -> Option<Self> {
        if self.id() != u32::MAX {
            Some(Self(unsafe { NonZeroU32::new_unchecked(self.id() + 1) }))
        } else {
            None
        }
    }
}
