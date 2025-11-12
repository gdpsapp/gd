use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, IntoOwned)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    #[default]
    Cube,
    Ship,
    Ball,
    Ufo,
    Wave,
    Robot,
    Spider,
    Swing,
    Jetpack,
}
