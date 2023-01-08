/// Adds chaining methods tap to every type.
pub trait Tap {
    /// Applies `f` to the value for its side effects, and returns the original value.
    ///
    /// # Examples
    /// ```
    /// use chaining::*;
    ///
    /// let xs = &[1, 2, 3].tap(|xs| println!("debug {}", xs.len()));
    /// assert_eq!(&[1, 2, 3], xs);
    /// ```
    fn tap<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: Fn(&Self),
    {
        f(&self);
        self
    }
}

impl<A> Tap for A {}
