// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Signature related errors used in this crate

use thiserror::Error as DeriveError;

use crate::crypto::key::KeyError;

pub use signing::MissingSignatureError;
pub use signing::SigningError;
pub use verifying::ProofValueError;
pub use verifying::VerificationError;
pub use verifying::VerificationProcessingError;

mod signing {
  use std::borrow::Cow;
  use std::fmt::Display;

  use crate::convert::JsonDecodingError;
  use crate::convert::JsonEncodingError;

  use super::DeriveError;
  use super::KeyError;
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
        description: format!("invalid input: serialization failed:: {}", error.to_string()).into(),
      }
    }
  }

  impl From<JsonDecodingError> for SigningError {
    fn from(error: JsonDecodingError) -> Self {
      Self {
        description: format!("invalid input: serialization failed:: {}", error.to_string()).into(),
      }
    }
  }

  impl From<KeyError> for SigningError {
    fn from(error: KeyError) -> Self {
      Self {
        description: format!("invalid input:: {}", error.0).into(), /* We will make this more efficient once const
                                                                     * evaluation has matured */
      }
    }
  }

  impl From<MissingSignatureError> for SigningError {
    fn from(_: MissingSignatureError) -> Self {
      Self {
        description: "unable to access the required signature:: signature not found".into(),
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
  use super::DeriveError;
  use super::KeyError;
  use super::MissingSignatureError;
  use crate::convert::JsonDecodingError;
  use crate::convert::JsonEncodingError;
  use crate::crypto::merkle_key::MerkleDigestKeyTagError;
  use crate::crypto::merkle_key::MerkleSignatureKeyTagError;
  /// The provided signature does not match the expected value
  #[derive(Debug, DeriveError)]
  #[error("{0}")]
  pub struct ProofValueError(pub &'static str);

  // Verification can typically fail by either actually verifying that the proof value is incorrect, or it can fail
  // before it gets to checking the proof value by for instance failing to (de)serialize some data etc. Hence the
  // verification error has two variants, where the latter wraps a private type.
  #[derive(Debug, DeriveError)]
  /// Caused by a failure to verify a cryptographic signature
  pub enum VerificationError {
    /// The provided signature does not match the expected value
    #[error("verification failed - invalid proof value: {0}")]
    InvalidProofValue(#[from] ProofValueError),

    /// Processing of the proof material failed before the proof value could be checked
    #[error("verification failed - processing failed before the proof value could be checked: {0}")]
    ProcessingFailed(#[from] VerificationProcessingError),
  }

  impl From<ProcessingErrorCause> for VerificationError {
    fn from(err: ProcessingErrorCause) -> Self {
      Self::ProcessingFailed(VerificationProcessingError::from(err))
    }
  }

  /// Indicates that something went wrong during a signature verification process before one could check the validity of
  /// the signature.
  #[derive(Debug, DeriveError)]
  #[error("{cause}")]
  pub struct VerificationProcessingError {
    #[from]
    cause: ProcessingErrorCause,
  }

  // This type gets wrapped in the public VerificationError type, hence we implement the Error trait in order to help
  // users with debugging and error logging.
  #[derive(Debug, DeriveError)]
  pub(crate) enum ProcessingErrorCause {
    // The format of the input to the verifier is not provided in the required format
    #[error("invalid input format:: {0}")]
    InvalidInputFormat(&'static str),
    // Unable to find the required signature
    #[error("missing signature:: {0}")]
    MissingSignature(&'static str),
    // Caused by attempting to parse an invalid Merkle Digest Key Collection tag.
    #[error(transparent)]
    InvalidMerkleDigestKeyTag(#[from] MerkleDigestKeyTagError),
    // Caused by attempting to parse an invalid Merkle Signature Key Collection tag.
    #[error(transparent)]
    InvalidMerkleSignatureKeyTag(#[from] MerkleSignatureKeyTagError),
    // Any other reason why the verification process failed
    #[error("{0}")]
    Other(&'static str),
  }

  impl From<JsonEncodingError> for VerificationError {
    fn from(_: JsonEncodingError) -> Self {
      VerificationError::from(ProcessingErrorCause::InvalidInputFormat("serialization failed"))
    }
  }

  impl From<JsonDecodingError> for VerificationError {
    fn from(_: JsonDecodingError) -> Self {
      VerificationError::from(ProcessingErrorCause::InvalidInputFormat("deserialization failed"))
    }
  }

  impl From<MissingSignatureError> for VerificationError {
    fn from(_: MissingSignatureError) -> Self {
      Self::ProcessingFailed(ProcessingErrorCause::MissingSignature("").into())
    }
  }
  impl From<KeyError> for VerificationError {
    fn from(err: KeyError) -> Self {
      Self::ProcessingFailed(ProcessingErrorCause::InvalidInputFormat(err.0).into())
    }
  }

  impl From<MerkleDigestKeyTagError> for VerificationProcessingError {
    fn from(err: MerkleDigestKeyTagError) -> Self {
      Self { cause: err.into() }
    }
  }

  impl From<MerkleSignatureKeyTagError> for VerificationProcessingError {
    fn from(err: MerkleSignatureKeyTagError) -> Self {
      Self { cause: err.into() }
    }
  }

  impl From<&'static str> for VerificationProcessingError {
    fn from(message: &'static str) -> Self {
      Self {
        cause: ProcessingErrorCause::Other(message),
      }
    }
  }
  impl TryFrom<VerificationError> for ProofValueError {
    type Error = &'static str;
    fn try_from(value: VerificationError) -> Result<Self, Self::Error> {
      match value {
        VerificationError::InvalidProofValue(err) => Ok(err),
        _ => Err("processing failed before the proof value could be checked"),
      }
    }
  }
}
