use once_cell::sync::Lazy;
use std::{fmt::Display, ops::Deref};

type InternalLazy = &'static str;

pub struct LazyStr(Lazy<InternalLazy>);

impl LazyStr {
    pub const fn new(f: fn() -> InternalLazy) -> Self {
        Self(Lazy::<InternalLazy>::new(f))
    }

    pub fn into_value<F>(this: LazyStr) -> Result<&'static str, fn() -> &'static str> {
        let val = this.0;
        once_cell::sync::Lazy::<&'static str>::into_value(val)
    }
}

impl Deref for LazyStr {
    type Target = InternalLazy;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Display for LazyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.deref())
    }
}
