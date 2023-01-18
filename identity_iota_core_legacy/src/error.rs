// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  #[error("{0}")]
  DiffError(#[from] identity_core::diff::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::Error),
  #[error("{0}")]
  InvalidDoc(#[from] identity_document::Error),
  #[error("Invalid Message: {0}")]
  InvalidMessage(#[from] bee_message::Error),

  #[error("signing failed: {0}")]
  DocumentSignError(&'static str, #[source] Option<identity_core::Error>),
  #[error("Invalid Document - Missing Message Id")]
  InvalidDocumentMessageId,
  #[error("Invalid Document - Signing Verification Method Type Not Supported")]
  InvalidDocumentSigningMethodType,
  #[error("Invalid Network Name")]
  InvalidNetworkName,
  #[error("invalid root document: {0}")]
  InvalidRootDocument(&'static str),
  #[error("Missing Signing Key")]
  MissingSigningKey,
  #[error("credential revocation error")]
  RevocationError(#[source] identity_did::Error),

  //New errors from active identity dependencies can go here to pass CI checks
  #[error("")]
  Compatibility,
}
