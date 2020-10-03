pub type Result<T, E = Error> = core::result::Result<T, E>;

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
  #[error("Missing Claim: {0}")]
  MissingClaim(&'static str),
  #[error("Invalid Claim: {0}")]
  InvalidClaim(&'static str),
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
pub enum EncodeError {
  #[error("Invalid Content: {0}")]
  InvalidContent(&'static str),
  #[error("Invalid Header Parameter: {0}")]
  InvalidParam(&'static str),
  #[error("Missing Header Parameter: {0}")]
  MissingParam(&'static str),
  #[error("Invalid Encoder Configuration: {0}")]
  InvalidConfiguration(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
  #[error("Invalid Input")]
  InvalidInput,
  #[error("Invalid Header Parameter: {0}")]
  InvalidParam(&'static str),
  #[error("Missing Header Parameter: {0}")]
  MissingParam(&'static str),
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
  #[error("Invalid Key Format: {0}")]
  InvalidKeyFormat(&'static str),
  #[error(transparent)]
  CryptoError(#[from] crypto::Error),
  #[error(transparent)]
  PemError(#[from] PemError),
  #[error(transparent)]
  ValidationError(#[from] ValidationError),
  #[error(transparent)]
  EncodeError(#[from] EncodeError),
  #[error(transparent)]
  DecodeError(#[from] DecodeError),
}
