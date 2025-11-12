use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time::SystemTime,
};

use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Integer = u32;
pub type Milliseconds = u64;

pub const ZERO: Milliseconds = 0;
pub const LIMIT: Milliseconds = Milliseconds::MAX;

pub const NOW: &str = "failed to get the timestamp";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum Error {
    #[error("system time is before the epoch")]
    Now,
    #[error("can not convert to milliseconds")]
    Milliseconds,
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
#[repr(transparent)]
#[serde(transparent)]
pub struct Duration {
    milliseconds: Milliseconds,
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Duration {
    pub const fn new(milliseconds: Milliseconds) -> Self {
        Self { milliseconds }
    }

    pub const fn get(self) -> Milliseconds {
        self.milliseconds
    }

    pub const fn checked_add(self, other: Self) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_add(other.get()) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_sub(other.get()) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn checked_mul(self, integer: Integer) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_mul(integer as Milliseconds) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn checked_div(self, integer: Integer) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_div(integer as Milliseconds) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn saturating_add(self, other: Self) -> Self {
        Self::new(self.get().saturating_add(other.get()))
    }

    pub const fn saturating_sub(self, other: Self) -> Self {
        Self::new(self.get().saturating_sub(other.get()))
    }

    pub const fn saturating_mul(self, integer: Integer) -> Self {
        Self::new(self.get().saturating_mul(integer as Milliseconds))
    }

    pub const fn saturating_div(self, integer: Integer) -> Self {
        Self::new(self.get().saturating_div(integer as Milliseconds))
    }

    pub const fn strict_add(self, other: Self) -> Self {
        Self::new(self.get().strict_add(other.get()))
    }

    pub const fn strict_sub(self, other: Self) -> Self {
        Self::new(self.get().strict_sub(other.get()))
    }

    pub const fn strict_mul(self, integer: Integer) -> Self {
        Self::new(self.get().strict_mul(integer as Milliseconds))
    }

    pub const fn strict_div(self, integer: Integer) -> Self {
        Self::new(self.get().strict_div(integer as Milliseconds))
    }

    pub const ZERO: Self = Self::new(ZERO);
    pub const LIMIT: Self = Self::new(LIMIT);
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.strict_add(other)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.strict_sub(other)
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<Integer> for Duration {
    type Output = Self;

    fn mul(self, integer: Integer) -> Self::Output {
        self.strict_mul(integer)
    }
}

impl MulAssign<Integer> for Duration {
    fn mul_assign(&mut self, integer: Integer) {
        *self = *self * integer;
    }
}

impl Div<Integer> for Duration {
    type Output = Self;

    fn div(self, integer: Integer) -> Self::Output {
        self.strict_div(integer)
    }
}

impl DivAssign<Integer> for Duration {
    fn div_assign(&mut self, integer: Integer) {
        *self = *self / integer;
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
#[repr(transparent)]
#[serde(transparent)]
pub struct Timestamp {
    milliseconds: Milliseconds,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Timestamp {
    pub const fn new(milliseconds: Milliseconds) -> Self {
        Self { milliseconds }
    }

    pub const fn get(self) -> Milliseconds {
        self.milliseconds
    }

    pub fn try_now() -> Result<Self, Error> {
        let time = SystemTime::now();

        let duration = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| Error::Now)?;

        let milliseconds = duration
            .as_millis()
            .try_into()
            .map_err(|_| Error::Milliseconds)?;

        Ok(Self::new(milliseconds))
    }

    pub fn now() -> Self {
        Self::try_now().expect(NOW)
    }

    pub const fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        if let Some(milliseconds) = self.get().checked_sub(earlier.get()) {
            Some(Duration::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn saturing_duration_since(self, earlier: Self) -> Duration {
        Duration::new(self.get().saturating_sub(earlier.get()))
    }

    pub const fn strict_duration_since(self, earlier: Self) -> Duration {
        Duration::new(self.get().strict_sub(earlier.get()))
    }

    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_add(duration.get()) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        if let Some(milliseconds) = self.get().checked_sub(duration.get()) {
            Some(Self::new(milliseconds))
        } else {
            None
        }
    }

    pub const fn saturating_add(self, duration: Duration) -> Self {
        Self::new(self.get().saturating_add(duration.get()))
    }

    pub const fn saturating_sub(self, duration: Duration) -> Self {
        Self::new(self.get().saturating_sub(duration.get()))
    }

    pub const fn strict_add(self, duration: Duration) -> Self {
        Self::new(self.get().strict_add(duration.get()))
    }

    pub const fn strict_sub(self, duration: Duration) -> Self {
        Self::new(self.get().strict_sub(duration.get()))
    }

    pub const ZERO: Self = Self::new(ZERO);
    pub const LIMIT: Self = Self::new(LIMIT);
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        self.strict_add(duration)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self.strict_sub(duration)
    }
}

impl SubAssign<Duration> for Timestamp {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.strict_duration_since(other)
    }
}
