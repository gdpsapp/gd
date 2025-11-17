use std::fmt;

use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::time::Duration;

pub const LIMIT: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("invalid percent `{value}`")]
pub struct PercentError {
    value: u8,
}

impl PercentError {
    pub(crate) const fn new(value: u8) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u8 {
        self.value
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    IntoOwned,
)]
pub struct Percent {
    value: u8,
}

impl fmt::Display for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value}%", value = self.get())
    }
}

impl Percent {
    pub const fn try_new(value: u8) -> Result<Self, PercentError> {
        if let Some(percent) = Self::new(value) {
            Ok(percent)
        } else {
            Err(PercentError::new(value))
        }
    }

    pub const fn new(value: u8) -> Option<Self> {
        if value > LIMIT {
            None
        } else {
            Some(unsafe { Self::new_unchecked(value) })
        }
    }

    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u8 {
        self.value
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, IntoOwned,
)]
#[serde(untagged)]
pub enum Record {
    Percent(Percent),
    Duration(Duration),
}
