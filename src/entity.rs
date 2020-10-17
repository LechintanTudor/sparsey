pub const INVALID_INDEX: u32 = u32::MAX;

#[derive(Copy, Clone, Debug)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    pub(crate) fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub fn invalid() -> Self {
        Self {
            index: INVALID_INDEX,
            generation: 0,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}
