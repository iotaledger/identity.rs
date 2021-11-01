use crate::error::Error::InvalidMessageFlags;
use crate::tangle::compression_brotli2::{compress_brotli2, decompress_brotli2};
use crate::Error;

#[derive(Copy, Clone)]
pub(crate) enum MessageEncodingVersion {
  JsonBrotli = 1,
}

static CURRENT_ENCODING_VERSION: MessageEncodingVersion = MessageEncodingVersion::JsonBrotli;

pub(crate) fn add_encoding_version_flag(mut compressed_data: Vec<u8>) -> Vec<u8> {
  let encoding_version = CURRENT_ENCODING_VERSION as u8;

  compressed_data.splice(0..0, [encoding_version].iter().cloned());
  compressed_data
}

pub(crate) fn get_decompressed_message_data<T: AsRef<[u8]>>(encoding_flag: u8, data: T) -> Result<String, Error> {
  return if encoding_flag == MessageEncodingVersion::JsonBrotli as u8 {
    decompress_brotli2(data.as_ref())
  } else {
    Err(InvalidMessageFlags)
  };
}

pub(crate) fn compress_message<T: AsRef<[u8]>>(message: T) -> Result<Vec<u8>, Error> {
  match CURRENT_ENCODING_VERSION {
    MessageEncodingVersion::JsonBrotli => compress_brotli2(message),
  }
}
