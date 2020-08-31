use base64::{decode_config, encode_config, URL_SAFE};

use crate::error::Result;

pub fn encode_b64(data: &(impl AsRef<[u8]> + ?Sized)) -> String {
  encode_config(data, URL_SAFE)
}

pub fn decode_b64(data: &(impl AsRef<str> + ?Sized)) -> Result<Vec<u8>> {
  decode_config(data.as_ref(), URL_SAFE).map_err(Into::into)
}
