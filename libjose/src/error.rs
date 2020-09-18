pub type Result<T, E = Error> = core::result::Result<T, E>;

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
  #[error("Invalid Claims: {0}")]
  InvalidClaims(anyhow::Error),
  #[error("Crypto Error: {0}")]
  CryptoError(anyhow::Error),
  #[error("Encode Error: {0}")]
  EncodeError(anyhow::Error),
  #[error("Decode Error: {0}")]
  DecodeError(anyhow::Error),
}

impl Error {
  pub fn invalid_key() -> Self {
    Self::CryptoError(anyhow!("Invalid Key"))
  }

  pub fn invalid_sig(algorithm: &'static str) -> Self {
    Self::CryptoError(anyhow!("Invalid Signature: {}", algorithm))
  }
}

#[cfg(feature = "ring-core")]
impl From<ring::error::Unspecified> for Error {
  fn from(_: ring::error::Unspecified) -> Self {
    Self::CryptoError(anyhow!("Unspecified"))
  }
}

#[cfg(feature = "ring-core")]
impl From<ring::error::KeyRejected> for Error {
  fn from(_: ring::error::KeyRejected) -> Self {
    Self::invalid_key()
  }
}

#[cfg(feature = "secp256k1")]
impl From<secp256k1::Error> for Error {
  fn from(_: secp256k1::Error) -> Self {
    Self::invalid_sig("ES256K")
  }
}
