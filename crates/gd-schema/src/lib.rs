pub mod artist;
pub mod option;
pub mod schema;
// pub mod user;

pub use schema::{FromReader, Schema, ToBuilder};

pub(crate) mod artist_capnp;
pub(crate) mod option_capnp;
// pub(crate) mod user_capnp;
