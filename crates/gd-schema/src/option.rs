use capnp::Error;

use crate::{
    option_capnp::option,
    schema::{FromReader, Schema, ToBuilder},
};

impl<'a, T: Schema<'a>> FromReader<'a> for Option<T> {
    type Reader = option::Reader<'a, T::Owned>;

    fn from_reader(reader: Self::Reader) -> Result<Self, Error> {
        let option::Which::Value(result) = reader.which()? else {
            return Ok(None);
        };

        let value = T::from_reader(result?)?;

        Ok(Some(value))
    }
}

impl<'a, T: Schema<'a>> ToBuilder<'a> for Option<T> {
    type Builder = option::Builder<'a, T::Owned>;

    fn to_builder(&self, mut builder: Self::Builder) -> Result<(), Error> {
        if let Some(value) = self {
            value.to_builder(builder.init_value())?;
        } else {
            builder.set_empty(());
        };

        Ok(())
    }
}

impl<'a, T: Schema<'a>> Schema<'a> for Option<T> {
    type Owned = option::Owned<T::Owned>;
}
