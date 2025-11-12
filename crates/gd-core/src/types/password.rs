use std::fmt;

use ownership::IntoOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("invalid code `{value}`")]
pub struct CodeError {
    value: u32,
}

impl CodeError {
    pub(crate) const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("unexpected negative value `{value}`")]
pub struct NegativeError {
    value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error(transparent)]
pub enum DeserializeError {
    Code(#[from] CodeError),
    Negative(#[from] NegativeError),
}

impl NegativeError {
    pub(crate) const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn get(self) -> i32 {
        self.value
    }
}

pub const NO_COPY: &str = "no copy";
pub const FREE_COPY: &str = "free copy";

pub const BASE: u32 = 10;

pub const DIGITS: u32 = 6;

pub const BOUND: u32 = BASE.pow(DIGITS);

pub const FREE: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub struct Code {
    value: u32,
}

impl fmt::Display for Code {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{value:align$}",
            value = self.get(),
            align = DIGITS as usize
        )
    }
}

impl Code {
    pub const fn try_new(value: u32) -> Result<Self, CodeError> {
        if let Some(code) = Self::new(value) {
            Ok(code)
        } else {
            Err(CodeError::new(value))
        }
    }

    pub const fn new(value: u32) -> Option<Self> {
        if value < BOUND {
            Some(unsafe { Self::new_unchecked(value) })
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u32 {
        self.value
    }
}

impl Serialize for Code {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u32::deserialize(deserializer)?;

        Self::try_new(value).map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub enum Password {
    #[default]
    NoCopy,
    FreeCopy,
    CodeCopy(Code),
}

impl Serialize for Password {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.into_serialize().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let option = Option::<i32>::deserialize(deserializer)?;

        Self::from_deserialize(option).map_err(D::Error::custom)
    }
}

impl fmt::Display for Password {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCopy => NO_COPY.fmt(formatter),
            Self::FreeCopy => FREE_COPY.fmt(formatter),
            Self::CodeCopy(code) => code.fmt(formatter),
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

    pub const fn is_code_copy(self) -> bool {
        matches!(self, Self::CodeCopy(_))
    }

    pub const fn code(self) -> Option<Code> {
        if let Self::CodeCopy(code) = self {
            Some(code)
        } else {
            None
        }
    }

    pub(crate) const fn into_serialize(self) -> Option<i32> {
        match self {
            Self::NoCopy => None,
            Self::FreeCopy => Some(FREE),
            Self::CodeCopy(code) => Some(code.get() as i32),
        }
    }

    pub(crate) const fn handle_signed(value: i32) -> Result<Option<u32>, NegativeError> {
        if value < 0 {
            if value == FREE {
                Ok(None)
            } else {
                Err(NegativeError::new(value))
            }
        } else {
            Ok(Some(value as u32))
        }
    }

    pub(crate) const fn from_deserialize(option: Option<i32>) -> Result<Self, DeserializeError> {
        let Some(value) = option else {
            return Ok(Self::NoCopy);
        };

        match Self::handle_signed(value) {
            Ok(None) => Ok(Self::FreeCopy),
            Ok(Some(code_value)) => match Code::try_new(code_value) {
                Ok(code) => Ok(Self::CodeCopy(code)),
                Err(error) => Err(DeserializeError::Code(error)),
            },
            Err(negative) => Err(DeserializeError::Negative(negative)),
        }
    }
}
