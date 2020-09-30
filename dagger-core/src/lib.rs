/// Provides instances of T.
///
/// Typically implemented by an injector.
pub trait Provider<T> {
    /// Provides a fully-constructed and injected instance of T.
    fn get(&mut self) -> T;
}

/// A handle to a lazily-computed value.
///
/// Each Lazy computes its value on the first call to get()
/// and remembers that same value for all subsequent calls to get().
pub trait Lazy<T> {
    /// Return the underlying value, computing the value if necessary.
    fn get(&mut self) -> T;
}
