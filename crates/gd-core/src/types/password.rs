use std::fmt;

use ownership::IntoOwned;

// TODO: (de)serialization

pub const NO_COPY: &str = "no copy";
pub const FREE_COPY: &str = "free copy";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub enum Password {
    #[default]
    NoCopy,
    FreeCopy,
    LockedCopy(u32),
}

impl fmt::Display for Password {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Password::NoCopy => NO_COPY.fmt(formatter),
            Password::FreeCopy => FREE_COPY.fmt(formatter),
            Password::LockedCopy(value) => value.fmt(formatter),
        }
    }
}

impl Password {
    pub const fn is_no_copy(self) -> bool {
        matches!(self, Self::NoCopy)
    }

    pub const fn is_free_copy(self) -> bool {
        matches!(self, Self::FreeCopy)
    }

    pub const fn is_locked_copy(self) -> bool {
        matches!(self, Self::LockedCopy(_))
    }

    pub const fn get(self) -> Option<u32> {
        if let Self::LockedCopy(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
