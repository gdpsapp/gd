use capnp::Error;
use gd_core::types::id::ArtistId;
use gd_entities::artists::Artist;

use crate::{
    artist_capnp::artist,
    schema::{FromReader, ToBuilder},
};

impl<'a> FromReader<'a> for Artist<'a> {
    type Reader = artist::Reader<'a>;

    fn from_reader(reader: Self::Reader) -> Result<Self, Error> {
        let id = reader.get_id();
        let name = reader.get_name()?.to_str()?;
        let verified = reader.get_verified();

        let artist_id = ArtistId::new(id);

        let artist = Self::builder()
            .id(artist_id)
            .name(name)
            .verified(verified)
            .build();

        Ok(artist)
    }
}

impl<'a> ToBuilder<'a> for Artist<'a> {
    type Builder = artist::Builder<'a>;

    fn to_builder(&self, builder: &mut Self::Builder) -> Result<(), Error> {
        builder.set_id(self.id.get());
        builder.set_name(self.name.as_ref());
        builder.set_verified(self.is_verified());

        Ok(())
    }
}
