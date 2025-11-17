use std::io::{Error, Read};

use flate2::{
    Compress, Compression, Decompress,
    bufread::{ZlibDecoder, ZlibEncoder},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to compress")]
pub struct CompressError(#[from] pub Error);

#[derive(Debug, Error)]
#[error("failed to decompress")]
pub struct DecompressError(#[from] pub Error);

pub const MAX_WINDOW_BITS: u8 = 15;

pub fn compressor() -> Compress {
    Compress::new_gzip(Compression::best(), MAX_WINDOW_BITS)
}

pub fn decompressor() -> Decompress {
    Decompress::new_gzip(MAX_WINDOW_BITS)
}

pub fn compress<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, CompressError> {
    fn compress_inner(data: &[u8]) -> Result<Vec<u8>, CompressError> {
        let mut encoder = ZlibEncoder::new_with_compress(data, compressor());

        let mut compressed = Vec::new();

        encoder.read_to_end(&mut compressed)?;

        Ok(compressed)
    }

    compress_inner(data.as_ref())
}

pub fn decompress<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, DecompressError> {
    fn decompress_inner(data: &[u8]) -> Result<Vec<u8>, DecompressError> {
        let mut decoder = ZlibDecoder::new_with_decompress(data, decompressor());

        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }

    decompress_inner(data.as_ref())
}
