use crate::error::{Error, Result};

pub fn decode_b58(data: &(impl AsRef<str> + ?Sized)) -> Result<Vec<u8>> {
    bs58::decode(data.as_ref()).into_vec().map_err(Error::DecodeBase58)
}

pub fn encode_b58(data: &(impl AsRef<[u8]> + ?Sized)) -> String {
    bs58::encode(data.as_ref()).into_string()
}

pub fn decode_hex(data: &(impl AsRef<str> + ?Sized)) -> Result<Vec<u8>> {
    hex::decode(data.as_ref()).map_err(Error::DecodeBase16)
}

pub fn encode_hex(data: &(impl AsRef<[u8]> + ?Sized)) -> String {
    hex::encode(data.as_ref())
}

pub fn decode_b64(data: &(impl AsRef<str> + ?Sized)) -> Result<Vec<u8>> {
    base64::decode_config(data.as_ref(), base64::URL_SAFE_NO_PAD).map_err(Error::DecodeBase64)
}

pub fn encode_b64(data: &(impl AsRef<[u8]> + ?Sized)) -> String {
    base64::encode_config(data.as_ref(), base64::URL_SAFE_NO_PAD)
}
