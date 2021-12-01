// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("failed to generate key-pair")]
  FailedKeyPairGeneration, //TODO: Remove this when this crate has its error types refactored
  /// Caused by a failure to produce a KeyCollection
  #[error("key collection error")]
  KeyCollectionError, // TODO: temporary solution to make this crate work with new errors from identity-core
  /// caused by a failure to deserialize a value
  #[error("deserialization error")]
  InvalidDeserialization, // TODO: temporary solution to make this crate work with new errors from identity-core
  /// caused by a failure to serialize a value
  #[error("serialization error")]
  InvalidSerialization, //TODO: temporary solution to make this crate work with new errors from identity-core
  /// caused by attempting to parse an invalid url
  #[error("url parsing error")]
  InvalidUrl, /* TODO: temporary solution to make this crate work
               * with new errors from iota-core */
  #[error("base decoding error")]
  BaseDecoding, // TODO: This is a temporary solution to make this crate work with the new errors from iota-core.
  #[error("error from the core crate. Remove this upon refactoring")]
  CoreError, // TODO: Remove this when refactoring the errors in this crate
  #[error("{0}")]
  DiffError(#[from] identity_core::diff::Error),
  #[error("{0}")]
  CredError(#[from] identity_credential::Error),
  #[error("{0}")]
  InvalidDID(#[from] identity_did::did::DIDError),
  #[error("{0}")]
  InvalidDoc(#[from] identity_did::Error),
  #[error("{0}")]
  ClientError(#[from] iota_client::error::Error),
  #[error("Invalid Message: {0}")]
  InvalidMessage(#[from] iota_client::bee_message::Error),
  #[error("{0}")]
  DIDNotFound(&'static str),
  #[error("Invalid Document - Missing Message Id")]
  InvalidDocumentMessageId,
  #[error("Invalid Document - Signing Verification Method Type Not Supported")]
  InvalidDocumentSigningMethodType,
  #[error("Invalid Verification Method - Missing Fragment")]
  InvalidMethodMissingFragment,
  #[error("Invalid Root Document")]
  InvalidRootDocument,
  #[error("Invalid Network Name")]
  InvalidNetworkName,
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
  #[error("Invalid Presentation Holder")]
  InvalidPresentationHolder,
  #[error("Chain Error: {error}")]
  ChainError { error: &'static str },
  #[error("Missing Signing Key")]
  MissingSigningKey,
  #[error("Cannot Revoke Verification Method")]
  CannotRevokeMethod,
  #[error("No Client Nodes Provided")]
  NoClientNodesProvided,
  #[error("No Explorer URL Set")]
  NoExplorerURLSet,
  #[error("Invalid Explorer Url")]
  InvalidExplorerURL,
  #[error("compression error")]
  CompressionError,
  #[error("invalid message flags")]
  InvalidMessageFlags,
}
