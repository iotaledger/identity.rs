pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
  #[cfg_attr(feature = "std", error("Invalid Base64: {0}"))]
  InvalidBase64(base64::DecodeError),
  #[cfg_attr(feature = "std", error("Missing Claim: {0}"))]
  MissingClaim(&'static str),
  #[cfg_attr(feature = "std", error("Invalid Claim: {0}"))]
  InvalidClaim(&'static str),
}

impl From<base64::DecodeError> for Error {
  fn from(other: base64::DecodeError) -> Self {
    Self::InvalidBase64(other)
  }
}
