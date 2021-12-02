// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Signature related errors used in this crate

use thiserror::Error as DeriveError;

use crate::crypto::key::KeyParsingError;

pub use signing::MissingSignatureError;
pub use signing::SigningError;
pub use verifying::VerificationError;

mod signing {
  use std::borrow::Cow;
  use std::fmt::Display;

  use crate::convert::JsonDecodingError;
  use crate::convert::JsonEncodingError;

  use super::DeriveError;
  use super::KeyParsingError;
  #[derive(Debug, DeriveError)]
  /// Caused by a failed attempt at retrieving a digital signature.
  #[error("signature not found")]
  pub struct MissingSignatureError;

  #[derive(Debug)]
  /// Caused by a failure to sign data
  pub struct SigningError {
    description: Cow<'static, str>,
  }

  impl From<JsonEncodingError> for SigningError {
    fn from(error: JsonEncodingError) -> Self {
      Self {
        description: format!("invalid input: serialization failed: {}", error.to_string()).into(),
      }
    }
  }

  impl From<JsonDecodingError> for SigningError {
    fn from(error: JsonDecodingError) -> Self {
      Self {
        description: format!("invalid input: serialization failed: {}", error.to_string()).into(),
      }
    }
  }

  impl From<KeyParsingError> for SigningError {
    fn from(error: KeyParsingError) -> Self {
      Self {
        description: format!("invalid input: {}", error.0).into(), /* might be possible to avoid allocating here in
                                                                    * the future */
      }
    }
  }

  impl From<MissingSignatureError> for SigningError {
    fn from(_: MissingSignatureError) -> Self {
      Self {
        description: "unable to access the required signature: signature not found".into(),
      }
    }
  }

  impl From<&'static str> for SigningError {
    fn from(err_str: &'static str) -> Self {
      Self {
        description: err_str.into(),
      }
    }
  }

  impl From<String> for SigningError {
    fn from(message: String) -> Self {
      Self {
        description: message.into(),
      }
    }
  }

  impl From<Cow<'static, str>> for SigningError {
    fn from(description: Cow<'static, str>) -> Self {
      Self { description }
    }
  }

  impl Display for SigningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "signing failed - {}", self.description)
    }
  }
}

mod verifying {
  use std::borrow::Cow;

  use super::DeriveError;
  use super::KeyParsingError;
  use super::MissingSignatureError;
  use crate::convert::JsonDecodingError;
  use crate::convert::JsonEncodingError;
  use crate::crypto::merkle_key::MerkleDigestKeyTagError;
  use crate::crypto::merkle_key::MerkleSignatureKeyTagError;

  #[derive(Debug, DeriveError)]
  /// Caused by a failure to verify a cryptographic signature
  pub enum VerificationError {
    /// The provided signature does not match the expected value.
    #[error("verification failed - invalid proof value: {0}")]
    InvalidProofValue(Cow<'static, str>),

    /// Caused by trying to verify a signature with a revoked key.
    #[error("verification failed - revoked: {0}")]
    Revoked(Cow<'static, str>),

    /// Processing of the proof material failed before the proof value could be checked.
    #[error("verification failed - processing failed before the proof value could be checked: {0}")]
    ProcessingFailed(Cow<'static, str>),
  }

  impl From<JsonEncodingError> for VerificationError {
    fn from(error: JsonEncodingError) -> Self {
      Self::ProcessingFailed(format!("invalid input format: serialization failed: {}", error.to_string()).into())
    }
  }

  impl From<JsonDecodingError> for VerificationError {
    fn from(error: JsonDecodingError) -> Self {
      Self::ProcessingFailed(format!("invalid input format: deserialization failed: {}", error).into())
    }
  }

  impl From<MissingSignatureError> for VerificationError {
    fn from(_: MissingSignatureError) -> Self {
      Self::ProcessingFailed("unable to access the required signature: signature not found".into())
    }
  }
  impl From<KeyParsingError> for VerificationError {
    fn from(error: KeyParsingError) -> Self {
      Self::ProcessingFailed(format!("invalid input format: {}", error.0).into()) // might be possible to avoid
                                                                                  // allocating here in the future
    }
  }

  impl From<MerkleDigestKeyTagError> for VerificationError {
    fn from(error: MerkleDigestKeyTagError) -> Self {
      Self::ProcessingFailed(error.to_string().into())
    }
  }

  impl From<MerkleSignatureKeyTagError> for VerificationError {
    fn from(error: MerkleSignatureKeyTagError) -> Self {
      Self::ProcessingFailed(error.to_string().into())
    }
  }
}
