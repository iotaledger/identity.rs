use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  InvalidJson(serde_json::Error),
  InvalidBase64(base64::DecodeError),
  InvalidDecompression(miniz_oxide::inflate::TINFLStatus),
  InvalidClaim(&'static str),
  MissingClaim(&'static str),
  InvalidParam(&'static str),
  MissingParam(&'static str),
  InvalidContent(&'static str),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::InvalidJson(inner) => f.write_fmt(format_args!("Invalid JSON: {}", inner)),
      Self::InvalidBase64(inner) => f.write_fmt(format_args!("Invalid Base64: {}", inner)),
      Self::InvalidDecompression(inner) => {
        f.write_fmt(format_args!("Invalid Decompression: {:?}", inner))
      }
      Self::InvalidClaim(inner) => f.write_fmt(format_args!("Invalid Claim: {}", inner)),
      Self::MissingClaim(inner) => f.write_fmt(format_args!("Missing Claim: {}", inner)),
      Self::InvalidParam(inner) => f.write_fmt(format_args!("Invalid Param: {}", inner)),
      Self::MissingParam(inner) => f.write_fmt(format_args!("Missing Param: {}", inner)),
      Self::InvalidContent(inner) => f.write_fmt(format_args!("Invalid Content: {}", inner)),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
  fn from(other: serde_json::Error) -> Self {
    Self::InvalidJson(other)
  }
}

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
