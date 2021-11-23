// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Signature related errors used in this crate 

use thiserror::Error as DeriveError;

use crate::crypto::key::KeyError; 

pub(crate) use signing::{MissingSignatureError, SigningError};
pub(crate) use signing::SigningErrorCause;
pub(crate) use verifying::{InvalidProofValue, VerificationError}; 
pub(crate) use verifying::VerificationProcessingErrorCause;


mod signing{
use super::{DeriveError,KeyError}; 
#[derive(Debug, DeriveError)]
/// Caused by a failed attempt at retrieving a digital signature.
#[error("signature not found")]
pub struct MissingSignatureError;

#[derive(Debug, DeriveError)]
// The reason why a signing operation failed. 
pub(crate) enum SigningErrorCause{
  // A signing operation failed because the signature could not be set
  #[error("signing failed - unable to access the required signature: {0}")]
  MissingAccess(&'static str),
  // Signing failed because the signing method received invalid input 
  #[error("signing failed - invalid input:  {0}")]
  Input(&'static str),
  // Any reason why signing failed that is not listed here 
  #[error("signing failed: {0}")]
  Other( &'static str),
}

#[derive(Debug, DeriveError)]
#[error("{cause}")]
/// Caused by a failure to sign data 
pub struct SigningError {
  cause: SigningErrorCause, 
}

impl From<SigningErrorCause> for SigningError {
  fn from(cause: SigningErrorCause) -> Self {
      Self{cause}
  }
}
impl From<KeyError> for SigningError {
  fn from(error: KeyError) -> Self {
      Self{ cause: SigningErrorCause::Input(error.0)}
  }
}

impl From<MissingSignatureError> for SigningError {
  fn from(_: MissingSignatureError) -> Self {
      Self{cause: SigningErrorCause::MissingAccess("signature missing")}
  }
}

impl From<&'static str> for SigningError {
  fn from(err_str: &'static str) -> Self {
      Self {cause: SigningErrorCause::Other(err_str)}
  }
}

impl<'a> AsRef<str> for SigningError {
  fn as_ref(&self) -> &str {
      match self.cause {
        SigningErrorCause::MissingAccess(err_str) => err_str,
        SigningErrorCause::Input(err_str) => err_str, 
        SigningErrorCause::Other(err_str) => err_str,
      }
  }
}
}


mod verifying {
use super::{DeriveError, KeyError, MissingSignatureError}; 
/// The provided signature does not match the expected value
#[derive(Debug, DeriveError)]
#[error("{0}")]
pub struct InvalidProofValue(pub &'static str);


// Verification can typically fail by either actually verifying that the proof value is incorrect, or it can fail before it gets to checking the proof value
// by for instance failing to (de)serialize some data etc. Hence the verification error has two variants, where the latter wraps a private type. 
#[derive(Debug, DeriveError)]
/// Caused by a failure to verify a cryptographic signature
pub enum VerificationError {
  /// The provided signature does not match the expected value
  #[error("verification failed - invalid proof value: {0}")]
  InvalidProofValue(#[from] InvalidProofValue), 
  
  /// Processing of the proof material failed before the proof value could be checked 
  #[error("verification failed - processing failed before the proof value could be checked: {0}")]
  ProcessingFailed(#[from] VerificationProcessingError), 
}

impl From<VerificationProcessingErrorCause> for VerificationError {
  fn from(err: VerificationProcessingErrorCause) -> Self {
      Self::ProcessingFailed(VerificationProcessingError::from(err))
  }
}

/// Indicates that something went wrong during a signature verification process before one could check the validity of the signature. 
#[derive(Debug, DeriveError)]
#[error("{cause}")]
pub struct VerificationProcessingError {
  #[from]
  cause: VerificationProcessingErrorCause,
}

// This type gets wrapped in the public VerificationError type, hence we implement the Error trait in order to help users with debugging and error logging. 
#[derive(Debug,DeriveError)]
pub (crate) enum VerificationProcessingErrorCause {
  // The format of the input to the verifier is not provided in the required format
  #[error("invalid input format:: {0}")]
  InvalidInputFormat(&'static str),
  // Unable to find the required signature 
  #[error("missing signature:: {0}")]
  MissingSignature(&'static str), 
  // Any other reason why the verification process failed 
  #[error("{0}")]
  Other(&'static str), 
}

impl From<MissingSignatureError> for VerificationError {
  fn from(_: MissingSignatureError) -> Self {
      Self::ProcessingFailed(VerificationProcessingErrorCause::MissingSignature("").into())
  }
}
impl From<KeyError> for VerificationError {
  fn from(err: KeyError) -> Self {
      Self::ProcessingFailed(VerificationProcessingErrorCause::InvalidInputFormat(err.0).into())
  }
}

impl TryFrom<VerificationError> for InvalidProofValue {
  type Error = &'static str; 
  fn try_from(value: VerificationError) -> Result<Self, Self::Error> {
      match value {
          VerificationError::InvalidProofValue(err) => Ok(err), 
          _ => Err("processing failed before the proof value could be checked")
      }
  }
}
}

      
