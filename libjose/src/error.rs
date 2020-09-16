pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid Base64: {0}")]
  InvalidBase64(#[from] base64::DecodeError),
}
