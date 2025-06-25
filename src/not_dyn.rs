//! Provider of [`NotDyn`].

/// Super trait for not dyn compatible traits.
pub trait NotDyn {
    /// Dummy method to make this trait not dyn compatible.
    #[doc(hidden)]
    fn __do_not_call() {}
}

impl<T: ?Sized> NotDyn for T {}
