use aes::{
    Aes256,
    cipher::{
        BlockDecryptMut, BlockEncryptMut, KeyInit,
        block_padding::{Pkcs7, UnpadError},
    },
};
use ecb;
use hex_literal::hex;
use thiserror::Error;

use crate::key;

#[derive(Debug, Error)]
#[error("failed to decrypt due to invalid padding")]
pub struct Error;

impl From<UnpadError> for Error {
    fn from(_unpad_error: UnpadError) -> Self {
        Self
    }
}

pub type Encryptor = ecb::Encryptor<Aes256>;
pub type Decryptor = ecb::Decryptor<Aes256>;

pub const BITS: usize = 256;

pub const BYTE_BITS: usize = u8::BITS as usize;

pub const SIZE: usize = BITS / BYTE_BITS;

pub type Data = key::Data<SIZE>;
pub type Key = key::Key<SIZE>;

pub const DATA: Data = hex!("69707539545576353479765d6973464d6835403b742e3577333445325279407b");

pub const KEY: Key = Key::new(DATA);

pub fn encryptor(key: Key) -> Encryptor {
    Encryptor::new(&key.get().into())
}

pub fn decryptor(key: Key) -> Decryptor {
    Decryptor::new(&key.get().into())
}

pub fn encrypt<D: AsRef<[u8]>>(data: D, key: Key) -> Vec<u8> {
    encryptor(key).encrypt_padded_vec_mut::<Pkcs7>(data.as_ref())
}

pub fn decrypt<D: AsRef<[u8]>>(data: D, key: Key) -> Result<Vec<u8>, Error> {
    let output = decryptor(key).decrypt_padded_vec_mut::<Pkcs7>(data.as_ref())?;

    Ok(output)
}
