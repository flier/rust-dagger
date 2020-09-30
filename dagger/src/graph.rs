use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectGraph<T> {
    phantom: PhantomData<T>,
}

pub fn create<T>() -> ObjectGraph<T> {
    ObjectGraph {
        phantom: PhantomData,
    }
}
