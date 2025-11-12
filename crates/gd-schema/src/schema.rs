use capnp::{
    Error,
    traits::{FromPointerBuilder, FromPointerReader, Owned},
};

pub trait FromReader<'a>: Sized {
    type Reader: FromPointerReader<'a>;

    fn from_reader(reader: Self::Reader) -> Result<Self, Error>;
}

pub trait ToBuilder<'a> {
    type Builder: FromPointerBuilder<'a>;

    fn to_builder(&self, builder: Self::Builder) -> Result<(), Error>;
}

pub trait Schema<'a>: FromReader<'a> + ToBuilder<'a> {
    type Owned: Owned<Reader<'a> = Self::Reader, Builder<'a> = Self::Builder>;
}
