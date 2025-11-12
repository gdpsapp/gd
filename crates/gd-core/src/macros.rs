#[doc(hidden)]
pub mod import {
    pub use std::fmt;

    pub use ownership::IntoOwned;
    pub use serde::{Deserialize, Serialize};
}

#[macro_export]
macro_rules! impl_untyped_id {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::types::id::Id for $type {
                type Type = $crate::types::id::Untyped;
            }
        )*
    };
}

impl_untyped_id!(u8, u16, u32, u64, u128);

#[macro_export]
macro_rules! impl_typed_id {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::types::id::Id for $type {
                type Type = $crate::types::id::Typed;
            }
        )*
    };
}

#[macro_export]
macro_rules! new_type {
    ($($name: ident => $type: ty),+ $(,)?) => {
        $(
            #[derive(
                Debug,
                Clone,
                Copy,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                Default,
                $crate::macros::import::Serialize,
                $crate::macros::import::Deserialize,
                $crate::macros::import::IntoOwned
            )]
            #[repr(transparent)]
            #[serde(transparent)]
            pub struct $name {
                value: $type,
            }

            impl $crate::macros::import::fmt::Display for $name {
                fn fmt(&self, formatter: &mut $crate::macros::import::fmt::Formatter<'_>) -> $crate::macros::import::fmt::Result {
                    self.get().fmt(formatter)
                }
            }

            impl $name {
                pub const fn new(value: $type) -> Self {
                    Self { value }
                }

                pub const fn get(self) -> $type {
                    self.value
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! custom_id {
    ($($name: ident => $untyped: ty),+ $(,)?) => {
        $(
            $crate::new_type!($name => $untyped);
            $crate::impl_typed_id!($name);

            impl $crate::types::id::CustomId for $name {
                type Untyped = $untyped;

                fn new(untyped: Self::Untyped) -> Self {
                    Self::new(untyped)
                }

                fn get(self) -> Self::Untyped {
                    self.get()
                }
            }
        )+
    };
}
