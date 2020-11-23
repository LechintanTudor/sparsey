const INVALID_ID: u32 = u32::MAX;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Entity {
    id: u32,
    gen: u32,
}

impl Entity {
    pub const INVALID: Self = Self::with_id(INVALID_ID);

    pub const fn new(id: u32, gen: u32) -> Self {
        Self { id, gen }
    }

    pub const fn with_id(id: u32) -> Self {
        Self { id, gen: 0 }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn gen(&self) -> u32 {
        self.gen
    }

    pub fn index(&self) -> usize {
        self.id as _
    }

    pub fn is_valid(&self) -> bool {
        self.id != INVALID_ID
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IndexEntity {
    id: u32,
    gen: u32,
}

impl IndexEntity {
    pub const INVALID: Self = Self::with_id(INVALID_ID);

    pub const fn new(id: u32, gen: u32) -> Self {
        Self { id, gen }
    }

    pub const fn with_id(id: u32) -> Self {
        Self { id, gen: 0 }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn gen(&self) -> u32 {
        self.gen
    }

    pub fn index(&self) -> usize {
        self.id as _
    }

    pub fn is_valid(&self) -> bool {
        self.id != INVALID_ID
    }
}
