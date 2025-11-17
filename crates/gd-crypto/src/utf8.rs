use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid utf-8")]
pub struct Error(#[from] pub FromUtf8Error);

pub fn convert(data: Vec<u8>) -> Result<String, Error> {
    let string = String::from_utf8(data)?;

    Ok(string)
}
