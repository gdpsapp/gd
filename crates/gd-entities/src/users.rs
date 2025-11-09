use std::fmt;

use bon::Builder;
use gd_core::types::{
    id::{AccountId, UserId},
    str::Str,
};
use ownership::IntoOwned;

use crate::entities::Entity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Builder, IntoOwned)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Builder, IntoOwned)]
pub struct User<'u> {
    pub reference: UserReference<'u>,
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
