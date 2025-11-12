use std::{hash::Hash, marker::PhantomData};

use ownership::IntoOwned;
use serde::{Serialize, de::DeserializeOwned};

mod sealed {
    pub trait Sealed {}
}

pub trait Type: sealed::Sealed {
    type Inverse: Type<Inverse = Self>;
}

pub struct Untyped {
    private: PhantomData<()>,
}

pub struct Typed {
    private: PhantomData<()>,
}

impl sealed::Sealed for Untyped {}
impl sealed::Sealed for Typed {}

impl Type for Untyped {
    type Inverse = Typed;
}

impl Type for Typed {
    type Inverse = Untyped;
}

pub trait Id: Copy + Ord + Hash + Default + Serialize + DeserializeOwned + IntoOwned {
    type Type: Type;
}

pub trait UntypedId: Id<Type = Untyped> {}
pub trait TypedId: Id<Type = Typed> {}

impl<T: Id<Type = Untyped>> UntypedId for T {}
impl<T: Id<Type = Typed>> TypedId for T {}

pub trait CustomId: TypedId {
    type Untyped: UntypedId;

    fn new(untyped: Self::Untyped) -> Self;
    fn get(self) -> Self::Untyped;
}

custom_id!(
    // large
    AccountId => u64,
    ArtistId => u64,
    UserId => u64,
    // small
    TimelyId => u16,
    ColorId => u16,
    IconId => u8,
    LongIconId => u16,
    RoleId => u8,
);
