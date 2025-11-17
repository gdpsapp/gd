use std::{fmt, time::Duration as StandardDuration};

use jiff::{SignedDuration, Timestamp};
use ownership::{IntoOwned, impl_identity};
use serde::{Deserialize, Serialize};

pub const UNREPRESENTABLE: &str = "unrepresentable";

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Instant {
    timestamp: Timestamp,
}

impl Instant {
    pub const fn new(timestamp: Timestamp) -> Self {
        Self { timestamp }
    }

    pub const fn get(self) -> Timestamp {
        self.timestamp
    }
}

impl_identity!(Instant);

// TODO: custom (de)serialization
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
pub struct Duration {
    standard: StandardDuration,
}

impl Duration {
    pub const fn new(standard: StandardDuration) -> Self {
        Self { standard }
    }

    pub const fn get(self) -> StandardDuration {
        self.standard
    }

    pub fn get_signed(self) -> Option<SignedDuration> {
        self.get().try_into().ok()
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(signed) = self.get_signed() {
            signed.fmt(formatter)
        } else {
            UNREPRESENTABLE.fmt(formatter)
        }
    }
}
