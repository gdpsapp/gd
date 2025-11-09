#[doc(hidden)]
pub mod import {
    pub use ownership::IntoOwned;
}

#[macro_export]
macro_rules! untyped_id {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::types::id::Id for $type {
                type Type = $crate::types::id::Untyped;
            }
        )*
    };
}

untyped_id!(u8, u16, u32, u64, u128);

#[macro_export]
macro_rules! typed_id {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::types::id::Id for $type {
                type Type = $crate::types::id::Typed;
            }
        )*
    };
}

#[macro_export]
macro_rules! custom_id {
    ($($name: ident => $untyped: ty),+ $(,)?) => {
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
                $crate::macros::import::IntoOwned,
            )]
            #[repr(transparent)]
            pub struct $name {
                value: $untyped,
            }

            impl $name {
                pub const fn new(value: $untyped) -> Self {
                    Self { value }
                }

                pub const fn get(self) -> $untyped {
                    self.value
                }
            }

            $crate::typed_id!($name);

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
