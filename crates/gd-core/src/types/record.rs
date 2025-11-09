use std::fmt;

use ownership::IntoOwned;
use thiserror::Error;

pub type Value = u8;

pub const LIMIT: Value = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("invalid percent `{value}`")]
pub struct Error {
    value: Value,
}

impl Error {
    pub(crate) const fn new(value: Value) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Value {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub struct Percent {
    value: u8,
}

impl fmt::Display for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value}%", value = self.get())
    }
}

impl Percent {
    pub const fn try_new(value: Value) -> Result<Self, Error> {
        if let Some(percent) = Self::new(value) {
            Ok(percent)
        } else {
            Err(Error::new(value))
        }
    }

    pub const fn new(value: Value) -> Option<Self> {
        if value > LIMIT {
            None
        } else {
            Some(unsafe { Self::new_unchecked(value) })
        }
    }

    pub const unsafe fn new_unchecked(value: Value) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Value {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoOwned)]
pub enum Record {
    Percent(Percent),
}

impl fmt::Display for Record {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percent(percent) => percent.fmt(formatter),
        }
    }
}
