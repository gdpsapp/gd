use gd_core::types::{
    id::{AccountId, UserId},
    str::Str,
};
use ownership::IntoOwned;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, IntoOwned)]
pub struct Credentials<'c> {
    pub name: Str<'c>,
    pub password: Str<'c>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub struct Reference {
    pub id: UserId,
    pub account_id: AccountId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, IntoOwned)]
pub struct Auth<'a> {
    pub credentials: Credentials<'a>,
    pub reference: Reference,
}
