use capnp::Error;
use gd_core::types::id::{AccountId, UserId};
use gd_entities::users::UserReference;

use crate::{
    schema::{FromReader, ToBuilder},
    user_capnp::user_reference,
};

impl<'u> FromReader<'u> for UserReference<'u> {
    type Reader = user_reference::Reader<'u>;

    fn from_reader(reader: Self::Reader) -> Result<Self, Error> {
        let id = reader.get_id();
        let name = reader.get_name()?.to_str()?;
        let account_id = reader.get_account_id();

        let user_id = UserId::new(id);
        let account_id = AccountId::new(account_id);

        let user_reference = UserReference::builder()
            .id(user_id)
            .name(name)
            .account_id(account_id)
            .build();

        Ok(user_reference)
    }
}

impl<'u> ToBuilder<'u> for UserReference<'u> {
    type Builder = user_reference::Builder<'u>;

    fn to_builder(&self, builder: &mut Self::Builder) -> Result<(), Error> {
        builder.set_id(self.id.get());
        builder.set_name(self.name.as_ref());
        builder.set_account_id(self.account_id.get());

        Ok(())
    }
}
