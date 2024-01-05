#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StorageMask(pub u16);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct QueryMask {
    pub include: StorageMask,
    pub exclude: StorageMask,
}
