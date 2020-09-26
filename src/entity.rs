#[derive(Copy, Clone, Debug)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn id(&self) -> u32 {
        self.id
    }
}
