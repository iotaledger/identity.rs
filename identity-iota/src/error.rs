// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("{0}")]
  CoreError(#[from] identity_core::Error),
  #[error("{0}")]
  DiffError(#[from] identity_core::diff::Error),
  #[error("{0}")]
  CredError(#[from] identity_credential::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::did::Error),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[error("{0}")]
  ClientError(#[from] iota_client::error::Error),
  #[error("Invalid Message: {0}")]
  InvalidMessage(#[from] iota_client::bee_message::Error),
  #[error("Invalid Document - Missing Message Id")]
  InvalidDocumentMessageId,
  #[error("Invalid Document - Authentication Authority Mismatch")]
  InvalidDocumentAuthAuthority,
  #[error("Invalid Document - Authentication Missing Fragment")]
  InvalidDocumentAuthFragment,
  #[error("Invalid Document - Authentication Type Not Supported")]
  InvalidDocumentAuthType,
  #[error("Invalid DID Network")]
  InvalidDIDNetwork,
  #[error("Invalid Tryte Conversion")]
  InvalidTryteConversion,
  #[error("Invalid Transaction Bundle")]
  InvalidTransactionBundle,
  #[error("Invalid Transaction Hashes")]
  InvalidTransactionHashes,
  #[error("Invalid Transaction Trytes")]
  InvalidTransactionTrytes,
  #[error("Invalid Bundle Tail")]
  InvalidBundleTail,
  #[error("Invalid PResentation Holder")]
  InvalidPresentationHolder,
  #[error("Chain Error: {error}")]
  ChainError { error: &'static str },
  #[error("Missing Verification Method Fragment")]
  MissingMethodFragment,
  #[error("Authentication Method Not Found")]
  MissingAuthenticationMethod,
  #[error("Cannot Remove Authentication Method")]
  CannotRemoveAuthMethod,
  #[error("Cannot Revoke Verification Method")]
  CannotRevokeMethod,
}
