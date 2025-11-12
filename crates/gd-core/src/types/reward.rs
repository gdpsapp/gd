use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::new_type;

new_type!(
    RewardCoins => u8,
    RewardStars => u8,
    RewardMoons => u8,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Reward {
    Stars(RewardStars),
    Moons(RewardMoons),
}
