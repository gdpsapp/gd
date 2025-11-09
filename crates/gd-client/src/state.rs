use ownership::IntoOwned;

use crate::auth::Auth;

mod sealed {
    pub trait Sealed {}
}

pub trait State: sealed::Sealed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub struct Simple;

impl Simple {
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, IntoOwned)]
pub struct Authenticated<'a> {
    pub auth: Auth<'a>,
}

impl<'a> Authenticated<'a> {
    pub const fn new(auth: Auth<'a>) -> Self {
        Self { auth }
    }
}

impl sealed::Sealed for Simple {}
impl sealed::Sealed for Authenticated<'_> {}

impl State for Simple {}
impl State for Authenticated<'_> {}
