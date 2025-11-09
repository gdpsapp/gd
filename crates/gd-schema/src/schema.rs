use capnp::{
    Error,
    traits::{FromPointerBuilder, FromPointerReader},
};

pub trait FromReader<'a>: Sized {
    type Reader: FromPointerReader<'a>;

    fn from_reader(reader: Self::Reader) -> Result<Self, Error>;
}

pub trait ToBuilder<'a> {
    type Builder: FromPointerBuilder<'a>;

    fn to_builder(&self, builder: &mut Self::Builder) -> Result<(), Error>;
}

pub trait Schema<'a>: FromReader<'a> + ToBuilder<'a> {}

impl<'a, T: FromReader<'a> + ToBuilder<'a>> Schema<'a> for T {}
