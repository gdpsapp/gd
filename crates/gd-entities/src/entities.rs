use std::fmt::Display;

use gd_core::types::id::CustomId;

pub trait Entity: Display {
    type Id: CustomId;

    fn id(&self) -> Self::Id;
}
