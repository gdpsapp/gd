#[doc(inline)]
pub use gd_core as core;

#[cfg(feature = "crypto")]
#[doc(inline)]
pub use gd_crypto as crypto;

#[cfg(feature = "entities")]
#[doc(inline)]
pub use gd_entities as entities;

#[cfg(feature = "enums")]
#[doc(inline)]
pub use gd_enums as enums;

#[cfg(feature = "images")]
#[doc(inline)]
pub use gd_images as images;

#[cfg(feature = "internals")]
#[doc(inline)]
pub use gd_internals as internals;

#[cfg(feature = "schema")]
#[doc(inline)]
pub use gd_schema as schema;
