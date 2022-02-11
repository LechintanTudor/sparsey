use crate::resources::Resources;
use crate::world::World;

/// Encapsulates a function with exclusive access to `World` and `Resources`.
pub struct LocalFn(Box<dyn FnMut(&mut World, &mut Resources) + 'static>);

impl LocalFn {
    /// Runs the local function.
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        (self.0)(world, resources);
    }
}

/// Helper trait for creating a local function from a function.
pub trait IntoLocalFn {
    /// Creates a local function.
    fn local_fn(self) -> LocalFn;
}

impl<F> IntoLocalFn for F
where
    F: FnMut(&mut World, &mut Resources) + 'static,
{
    fn local_fn(self) -> LocalFn {
        LocalFn(Box::new(self))
    }
}

impl IntoLocalFn for LocalFn {
    fn local_fn(self) -> LocalFn {
        self
    }
}
