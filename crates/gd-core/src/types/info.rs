use bon::Builder;
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::types::statistics::{Demons, Levels};

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
    Builder,
    Serialize,
    Deserialize,
    IntoOwned,
)]
#[serde(default)]
pub struct DemonInfoGroup {
    #[builder(default)]
    pub easy: Demons,
    #[builder(default)]
    pub medium: Demons,
    #[builder(default)]
    pub hard: Demons,
    #[builder(default)]
    pub insane: Demons,
    #[builder(default)]
    pub extreme: Demons,
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
    Builder,
    Serialize,
    Deserialize,
    IntoOwned,
)]
pub struct DemonInfoSpecial {
    #[builder(default)]
    pub weekly: Demons,
    #[builder(default)]
    pub gauntlet: Demons,
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
    Builder,
    Serialize,
    Deserialize,
    IntoOwned,
)]
pub struct DemonInfo {
    #[builder(default)]
    pub regular: DemonInfoGroup,
    #[builder(default)]
    pub platformer: DemonInfoGroup,
    #[builder(default)]
    pub special: DemonInfoSpecial,
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
    Builder,
    Serialize,
    Deserialize,
    IntoOwned,
)]
pub struct LevelInfo {
    #[builder(default)]
    pub auto: Levels,
    #[builder(default)]
    pub easy: Levels,
    #[builder(default)]
    pub normal: Levels,
    #[builder(default)]
    pub hard: Levels,
    #[builder(default)]
    pub harder: Levels,
    #[builder(default)]
    pub insane: Levels,
    #[builder(default)]
    pub daily: Levels,
    #[builder(default)]
    pub gauntlet: Levels,
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
    Builder,
    Serialize,
    Deserialize,
    IntoOwned,
)]
pub struct PlatformerInfo {
    #[builder(default)]
    pub auto: Levels,
    #[builder(default)]
    pub easy: Levels,
    #[builder(default)]
    pub normal: Levels,
    #[builder(default)]
    pub hard: Levels,
    #[builder(default)]
    pub harder: Levels,
    #[builder(default)]
    pub insane: Levels,
}
