pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
  #[error("Invalid Key Format: {0}")]
  InvalidKeyFormat(&'static str),
  #[error("Invalid Signature: {0}")]
  InvalidSignature(&'static str),
  #[error("Unspecified Error")]
  Unspecified,
}

#[derive(Debug, thiserror::Error)]
pub enum PemError {
  #[error("Invalid UTF-8: {0}")]
  InvalidUtf8(#[from] core::str::Utf8Error),
  #[error("Invalid PEM Header")]
  InvalidHeader,
  #[error("Invalid PEM Footer")]
  InvalidFooter,
  #[error("Invalid PEM Header/Footer")]
  InvalidHeaderFooter,
  #[error("Invalid PEM Content")]
  InvalidContent,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
  #[error("Missing Required Claim: {0}")]
  MissingClaim(&'static str),
  #[error("Invalid Audience Claim")]
  InvalidAudience,
  #[error("Invalid Issuer Claim")]
  InvalidIssuer,
  #[error("Invalid JWT ID Claim")]
  InvalidTokenId,
  #[error("Invalid Subject Claim")]
  InvalidSubject,
  #[error("Token Expired")]
  TokenExpired,
  #[error("Token Not Yet Valid")]
  TokenNotYetValid,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid Base64: {0}")]
  InvalidBase64(#[from] base64::DecodeError),
  #[error("Invalid JSON: {0}")]
  InvalidJson(#[from] serde_json::Error),
  #[error("Invalid JWK Format: {0}")]
  InvalidJwkFormat(anyhow::Error),
  #[error("Invalid JWS Format: {0}")]
  InvalidJwsFormat(anyhow::Error),
  #[error(transparent)]
  CryptoError(#[from] CryptoError),
  #[error(transparent)]
  PemError(#[from] PemError),
  #[error(transparent)]
  ValidationError(#[from] ValidationError),
  #[error("Encode Error: {0}")]
  EncodeError(anyhow::Error),
  #[error("Decode Error: {0}")]
  DecodeError(anyhow::Error),
}

#[cfg(feature = "ring-core")]
impl From<ring::error::Unspecified> for Error {
  fn from(_: ring::error::Unspecified) -> Self {
    Self::CryptoError(CryptoError::Unspecified)
  }
}

#[cfg(feature = "ring-core")]
impl From<ring::error::KeyRejected> for Error {
  fn from(_: ring::error::KeyRejected) -> Self {
    Self::CryptoError(CryptoError::InvalidKeyFormat("REJECTED"))
  }
}

#[cfg(feature = "secp256k1")]
impl From<secp256k1::Error> for Error {
  fn from(_: secp256k1::Error) -> Self {
    Self::CryptoError(CryptoError::InvalidSignature("ES256K"))
  }
}
