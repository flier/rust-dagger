pub use dagger_core::{Lazy, Provider};
pub use dagger_macros::{component, inject, module, Singleton};

mod graph;

pub use self::graph::{create, ObjectGraph};

pub trait Component<T> {
    type Builder: Builder<T>;

    fn builder() -> Self::Builder;
}

pub trait Builder<T> {
    fn build() -> T;
}
