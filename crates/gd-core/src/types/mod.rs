pub mod color;
pub mod id;
pub mod info;
pub mod password;
pub mod record;
pub mod reward;
pub mod statistics;
pub mod str;
pub mod time;
pub mod values;
pub mod version;

#[doc(inline)]
pub use color::{AlphaColor, Color, OpaqueColor};

#[doc(inline)]
pub use id::{AccountId, ArtistId, CustomId, Id, TypedId, UntypedId, UserId};

#[doc(inline)]
pub use info::{DemonInfo, DemonInfoGroup, DemonInfoSpecial, LevelInfo, PlatformerInfo};

#[doc(inline)]
pub use password::{Code, Password};

#[doc(inline)]
pub use record::{Percent, Record};

#[doc(inline)]
pub use reward::{Reward, RewardCoins, RewardMoons, RewardStars};

#[doc(inline)]
pub use statistics::{
    Attempts, Clicks, CreatorPoints, Demons, Diamonds, Downloads, Jumps, Levels, Moons,
    ObjectCount, Place, Rank, Rating, Score, SecretCoins, Stars, UserCoins,
};

#[doc(inline)]
pub use str::Str;

#[doc(inline)]
pub use time::Duration;

#[doc(inline)]
pub use values::{EnumValue, FlagValue};

#[doc(inline)]
pub use version::{Revision, Version};
