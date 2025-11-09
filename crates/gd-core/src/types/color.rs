use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

pub type Value = u32;

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
    IntoOwned,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Color {
    value: Value,
}

impl Color {
    pub const fn new(value: Value) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Value {
        self.value
    }
}
