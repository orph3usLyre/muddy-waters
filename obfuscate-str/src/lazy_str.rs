use once_cell::sync::Lazy;
use std::{fmt::Display, ops::Deref};

type InternalLazy = &'static str;

#[derive(Debug)]
pub struct LazyStr(Lazy<InternalLazy>);

impl LazyStr {
    /// Returns a new [`LazyStr`] given a construction function
    ///
    pub const fn new(f: fn() -> InternalLazy) -> Self {
        Self(Lazy::<InternalLazy>::new(f))
    }

    /// Returns the internal value by calling [`Lazy::into_value`] on it
    ///
    /// # Errors
    /// Check the corresponding [`once_cell::sync::Lazy`] for error information
    pub fn into_value<F>(this: LazyStr) -> Result<InternalLazy, fn() -> &'static str> {
        let val = this.0;
        once_cell::sync::Lazy::<&'static str>::into_value(val)
    }
}

impl Deref for LazyStr {
    type Target = InternalLazy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LazyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*self.0)
    }
}
