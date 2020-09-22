use crate::error::Result;

pub fn encode_b64(data: impl AsRef<[u8]>) -> String {
  base64::encode_config(data.as_ref(), base64::URL_SAFE_NO_PAD)
}

pub fn encode_b64_into(data: impl AsRef<[u8]>, buffer: &mut String) {
  base64::encode_config_buf(data.as_ref(), base64::URL_SAFE_NO_PAD, buffer)
}

pub fn decode_b64(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  base64::decode_config(data.as_ref(), base64::URL_SAFE_NO_PAD).map_err(Into::into)
}

pub fn decode_b64_into(data: impl AsRef<[u8]>, buffer: &mut Vec<u8>) -> Result<()> {
  base64::decode_config_buf(data.as_ref(), base64::URL_SAFE_NO_PAD, buffer).map_err(Into::into)
}
