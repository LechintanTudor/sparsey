use crate::resources::Resources;
use crate::world::World;

pub struct LocalFn(Box<dyn FnMut(&mut World, &mut Resources) + 'static>);

impl LocalFn {
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        (self.0)(world, resources);
    }
}

pub trait IntoLocalFn {
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
