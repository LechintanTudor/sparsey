pub trait Resource
where 
    Self: 'static,
{}

impl<T> Resource for T
where 
    T: 'static,
{}

pub struct Resources {
    
}
