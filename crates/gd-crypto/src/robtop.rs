use thiserror::Error;

use crate::{base64, utf8, xor};

#[derive(Debug, Error)]
#[error("failed to decode robtop into string")]
pub enum Error {
    Base64(#[from] base64::Error),
    Utf8(#[from] utf8::Error),
}

pub fn encode_mut<D: AsMut<[u8]>, K: AsRef<[u8]>>(mut data: D, key: K) -> String {
    fn encode_mut_inner(data: &mut [u8], key: &[u8]) -> String {
        xor::cyclic_xor(&mut *data, key); // reborrow to avoid move

        base64::encode(data)
    }

    encode_mut_inner(data.as_mut(), key.as_ref())
}

pub fn encode<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> String {
    encode_mut(data.as_ref().to_owned(), key)
}

pub fn decode<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> Result<Vec<u8>, base64::Error> {
    fn decode_inner(data: &[u8], key: &[u8]) -> Result<Vec<u8>, base64::Error> {
        let mut owned = base64::decode(data)?;

        xor::cyclic_xor(&mut owned, key);

        Ok(owned)
    }

    decode_inner(data.as_ref(), key.as_ref())
}

pub fn decode_string<D: AsRef<[u8]>, K: AsRef<[u8]>>(data: D, key: K) -> Result<String, Error> {
    fn decode_string_inner(data: &[u8], key: &[u8]) -> Result<String, Error> {
        let output = decode(data, key)?;

        let string = utf8::convert(output)?;

        Ok(string)
    }

    decode_string_inner(data.as_ref(), key.as_ref())
}
