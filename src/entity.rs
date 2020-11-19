pub const INVALID_ID: u32 = u32::MAX;

#[derive(Copy, Clone, Debug)]
pub struct Entity {
    id: u32,
    gen: u32,
}

impl Entity {
    pub const INVALID: Self = Self::new(INVALID_ID);

    pub const fn new(id: u32) -> Self {
        Self { id, gen: 0 }
    }

    pub const fn from_id_and_gen(id: u32, gen: u32) -> Self {
        Self { id, gen }
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
