use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, IntoOwned)]
#[serde(rename_all = "snake_case")]
pub enum State {
    #[default]
    OpenToAll,
    OpenToFriends,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    User,
    Level,
}
