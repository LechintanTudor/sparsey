use std::any::Any;

/// Trait automatically implemented for all types that can be stored in
/// [`Resources`](`crate::resources::Resources`)
pub trait Resource: Downcast {
    // Empty
}

impl<T> Resource for T
where
    T: Downcast,
{
    // Empty
}

impl dyn Resource {
    /// Returns whether the resource is of type `T`.
    #[must_use]
    pub fn is<T>(&self) -> bool
    where
        T: Resource,
    {
        self.as_any().is::<T>()
    }

    /// Tries to downcast `self` to a box of type `T`. If the conversion fails, the original box is
    /// returned.
    pub fn downcast<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>
    where
        T: Resource,
    {
        if self.is::<T>() {
            Ok(self.into_any().downcast().unwrap())
        } else {
            Err(self)
        }
    }

    /// Tries to downcast `self` to a reference of type `T`.
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Resource,
    {
        self.as_any().downcast_ref()
    }

    /// Tries to downcast `self` to a mutable reference of type `T`.
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Resource,
    {
        self.as_any_mut().downcast_mut()
    }
}

/// Helper trait for implementing downcasting operations.
pub trait Downcast: 'static {
    /// Returns `self` as a type-erases box.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    /// Returns `self` as a type-erased reference.
    fn as_any(&self) -> &dyn Any;

    /// Returns `self` as a type-erased mutable reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Downcast for T
where
    T: 'static,
{
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
