use crate::components::{Component, ComponentStorage};
use std::any;
use std::any::TypeId;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Holds information about a component type.
pub struct ComponentInfo {
	component: Box<dyn AbstractType>,
}

unsafe impl Send for ComponentInfo {}
unsafe impl Sync for ComponentInfo {}

impl Clone for ComponentInfo {
	fn clone(&self) -> Self {
		Self {
			component: self.component.clone(),
		}
	}
}

impl ComponentInfo {
	/// Creates a new `LayoutComponent` for the given component type.
	pub fn new<C>() -> Self
	where
		C: Component,
	{
		Self {
			component: Box::new(Type::<C>(PhantomData)),
		}
	}

	/// Returns the `TypeId` of the component.
	pub fn type_id(&self) -> TypeId {
		self.component.type_id()
	}

	/// Returns the type name of the component..
	pub fn type_name(&self) -> &'static str {
		self.component.type_name()
	}

	/// Returns the `TypeId` of the component and an empty `ComponentStorage`
	/// for that component.
	pub fn new_storage(&self) -> (TypeId, ComponentStorage) {
		self.component.new_storage()
	}
}

impl PartialEq for ComponentInfo {
	fn eq(&self, other: &Self) -> bool {
		self.type_id().eq(&other.type_id())
	}
}

impl Eq for ComponentInfo {}

impl PartialOrd for ComponentInfo {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.type_id().partial_cmp(&other.type_id())
	}
}

impl Ord for ComponentInfo {
	fn cmp(&self, other: &Self) -> Ordering {
		self.type_id().cmp(&other.type_id())
	}
}

impl Hash for ComponentInfo {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.type_id().hash(state);
	}
}

#[derive(Copy, Clone)]
struct Type<T>(PhantomData<*const T>);

unsafe impl<T> Send for Type<T> {}
unsafe impl<T> Sync for Type<T> {}

impl<T> Default for Type<T> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

unsafe trait AbstractType {
	fn type_id(&self) -> TypeId;

	fn type_name(&self) -> &'static str;

	fn new_storage(&self) -> (TypeId, ComponentStorage);

	fn clone(&self) -> Box<dyn AbstractType>;
}

unsafe impl<T> AbstractType for Type<T>
where
	T: Component,
{
	fn type_id(&self) -> TypeId {
		TypeId::of::<T>()
	}

	fn type_name(&self) -> &'static str {
		any::type_name::<T>()
	}

	fn new_storage(&self) -> (TypeId, ComponentStorage) {
		(self.type_id(), ComponentStorage::for_type::<T>())
	}

	fn clone(&self) -> Box<dyn AbstractType> {
		Box::new(Type::<T>::default())
	}
}
