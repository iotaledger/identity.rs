use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  InvalidBase64(base64::DecodeError),
  InvalidDecompression(miniz_oxide::inflate::TINFLStatus),
  MissingClaim(&'static str),
  InvalidClaim(&'static str),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::InvalidBase64(inner) => f.write_fmt(format_args!("Invalid Base64: {}", inner)),
      Self::InvalidDecompression(inner) => {
        f.write_fmt(format_args!("Invalid Decompression: {:?}", inner))
      }
      Self::MissingClaim(inner) => f.write_fmt(format_args!("Missing Claim: {}", inner)),
      Self::InvalidClaim(inner) => f.write_fmt(format_args!("Invalid Claim: {}", inner)),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<base64::DecodeError> for Error {
  fn from(other: base64::DecodeError) -> Self {
    Self::InvalidBase64(other)
  }
}

impl From<miniz_oxide::inflate::TINFLStatus> for Error {
  fn from(other: miniz_oxide::inflate::TINFLStatus) -> Self {
    Self::InvalidDecompression(other)
  }
}
