pub(crate) type StorageMask = u16;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub(crate) struct QueryMask {
    include: StorageMask,
    exclude: StorageMask,
}

impl QueryMask {
    pub const fn new(include: StorageMask, exclude: StorageMask) -> Self {
        Self { include, exclude }
    }

    pub const fn include(arity: usize) -> Self {
        Self {
            include: (1 << arity) - 1,
            exclude: 0,
        }
    }

    pub const fn exclude(prev_arity: usize, arity: usize) -> Self {
        if prev_arity != 0 {
            let exclude_count = arity - prev_arity;

            Self {
                include: (1 << prev_arity) - 1,
                exclude: ((1 << exclude_count) - 1) << prev_arity,
            }
        } else {
            Self::new(0, 0)
        }
    }
}
