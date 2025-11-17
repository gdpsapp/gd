use base64::{
    DecodeError,
    engine::{Engine, general_purpose::URL_SAFE},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to decode base64 data")]
pub struct Error(#[from] DecodeError);

pub fn encode<D: AsRef<[u8]>>(data: D) -> String {
    URL_SAFE.encode(data)
}

pub fn decode<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, Error> {
    let output = URL_SAFE.decode(data)?;

    Ok(output)
}
