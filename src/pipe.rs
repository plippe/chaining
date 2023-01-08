/// Adds chaining methods pipe to every type.
pub trait Pipe {
    /// Converts the value by applying the function `f`.
    ///
    /// # Examples
    /// ```
    /// use chaining::*;
    ///
    /// let times6 = |i: i8| i * 6;
    /// let i = (1 - 2 - 3).pipe(times6).pipe(i8::abs);
    ///
    /// assert_eq!(24, i);
    /// ```
    fn pipe<B, F>(self, f: F) -> B
    where
        Self: Sized,
        F: Fn(Self) -> B,
    {
        f(self)
    }
}

impl<A> Pipe for A {}
