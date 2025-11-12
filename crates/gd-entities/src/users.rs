use std::fmt;

use bon::Builder;
use gd_core::types::{
    id::{AccountId, ColorId, IconId, LongIconId, RoleId, UserId},
    info::{DemonInfo, LevelInfo, PlatformerInfo},
    record::Record,
    reward::RewardCoins,
    statistics::{
        CreatorPoints, Demons, Diamonds, Moons, Place, Rank, SecretCoins, Stars, UserCoins,
    },
    str::Str,
    time::Timestamp,
};
use gd_enums::{comments, icons};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::entities::Entity;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
pub struct UserReference<'u> {
    pub id: UserId,
    #[builder(into)]
    pub name: Str<'u>,
    pub account_id: AccountId,
}

impl fmt::Display for UserReference<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(formatter)
    }
}

impl Entity for UserReference<'_> {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
#[serde(default)]
pub struct UserStatistics {
    #[builder(default)]
    pub stars: Stars,
    #[builder(default)]
    pub moons: Moons,
    #[builder(default)]
    pub demons: Demons,
    #[builder(default)]
    pub diamonds: Diamonds,
    #[builder(default)]
    pub user_coins: UserCoins,
    #[builder(default)]
    pub secret_coins: SecretCoins,
    #[builder(default)]
    pub creator_points: CreatorPoints,
    #[builder(default)]
    pub rank: Rank,
    #[builder(default)]
    pub demon_info: DemonInfo,
    #[builder(default)]
    pub level_info: LevelInfo,
    #[builder(default)]
    pub platformer_info: PlatformerInfo,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
pub struct UserCosmetics {
    #[builder(default)]
    pub color_1_id: ColorId,
    #[builder(default)]
    pub color_2_id: ColorId,
    #[builder(default)]
    pub color_3_id: ColorId,
    #[builder(default)]
    pub glow: bool,
    #[builder(default)]
    pub icon_type: icons::Type,
    #[builder(default)]
    pub icon_id: LongIconId,
    #[builder(default)]
    pub cube_id: LongIconId,
    #[builder(default)]
    pub ship_id: IconId,
    #[builder(default)]
    pub ball_id: IconId,
    #[builder(default)]
    pub ufo_id: IconId,
    #[builder(default)]
    pub wave_id: IconId,
    #[builder(default)]
    pub robot_id: IconId,
    #[builder(default)]
    pub spider_id: IconId,
    #[builder(default)]
    pub swing_id: IconId,
    #[builder(default)]
    pub jetpack_id: IconId,
    #[builder(default)]
    pub explosion_id: IconId,
    #[builder(default)]
    pub streak_id: IconId,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
pub struct UserStates {
    #[builder(default)]
    pub comments: comments::State,
    // TODO: message, friend request, friend (?)
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
pub struct UserSocials<'u> {
    youtube: Option<Str<'u>>,
    x: Option<Str<'u>>,
    twitch: Option<Str<'u>>,
    discord: Option<Str<'u>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Serialize, Deserialize, IntoOwned)]
pub struct UserLeaderboard {
    record: Record,
    #[builder(default)]
    coins: RewardCoins,
    #[builder(default)]
    recorded_at: Timestamp,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, Builder, Serialize, Deserialize, IntoOwned,
)]
pub struct User<'u> {
    pub reference: UserReference<'u>,
    #[builder(default)]
    pub banned: bool,
    #[builder(default)]
    pub role_id: RoleId,
    pub statistics: Option<UserStatistics>,
    pub cosmetics: Option<UserCosmetics>,
    pub states: Option<UserStates>,
    pub socials: Option<UserSocials<'u>>,
    pub place: Option<Place>,
    pub leaderboard: Option<UserLeaderboard>,
}

impl fmt::Display for User<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reference.fmt(formatter)
    }
}

impl Entity for User<'_> {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.reference.id
    }
}

impl User<'_> {
    pub const fn is_banned(&self) -> bool {
        self.banned
    }
}
