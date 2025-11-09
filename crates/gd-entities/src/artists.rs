use std::fmt;

use bon::Builder;
use gd_core::types::{id::ArtistId, str::Str};
use ownership::IntoOwned;

use crate::entities::Entity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Builder, IntoOwned)]
pub struct Artist<'a> {
    pub id: ArtistId,
    #[builder(into)]
    pub name: Str<'a>,
    pub verified: bool,
}

impl Entity for Artist<'_> {
    type Id = ArtistId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl fmt::Display for Artist<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(formatter)
    }
}

impl Artist<'_> {
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}
