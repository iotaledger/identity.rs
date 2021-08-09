// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Decentralized Identifiers.

/// Alias for a [`Result`][::core::result::Result] with the error type [Error].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused by errors from the [identity_core] crate.
  #[error("{0}")]
  CoreError(#[from] ::identity_core::Error),
  /// Caused by errors from the [`did_url`][::did_url::Error] crate.
  #[error("{0}")]
  DIDError(#[from] ::did_url::Error),

  #[error("Duplicate Item in Ordered Set")]
  OrderedSetDuplicate,
  #[error("Verification Method Not Found")]
  QueryMethodNotFound,

  #[error("Invalid Document Property: `id`")]
  BuilderInvalidDocumentId,

  #[error("Invalid Service Property: `id`")]
  BuilderInvalidServiceId,
  #[error("Invalid Service Property: `type`")]
  BuilderInvalidServiceType,
  #[error("Invalid Service Property: `service_endpoint`")]
  BuilderInvalidServiceEndpoint,

  #[error("Invalid Verification Method Property: `id`")]
  BuilderInvalidMethodId,
  #[error("Invalid Verification Method Property: `controller`")]
  BuilderInvalidMethodController,
  #[error("Invalid Verification Method Property: `type`")]
  BuilderInvalidMethodType,
  #[error("Invalid Verification Method Property: `data`")]
  BuilderInvalidMethodData,

  #[error("Invalid Verification Method Fragment")]
  InvalidMethodFragment,
  #[error("Invalid Verification Method Type")]
  InvalidMethodType,
  #[error("Invalid Verification Method - Duplicate")]
  InvalidMethodDuplicate,

  #[error("Unknown Method Scope")]
  UnknownMethodScope,
  #[error("Unknown Method Type")]
  UnknownMethodType,
  #[error("Unknown Signature Type")]
  UnknownSignatureType,

  #[error("Invalid Key Data")]
  InvalidKeyData,
  #[error("Invalid Base16 Key Data")]
  InvalidKeyDataBase16,
  #[error("Invalid Base58 Key Data")]
  InvalidKeyDataBase58,
  #[error("Invalid Multibase Key Data")]
  InvalidKeyDataMultibase,

  #[error("Missing Resolution DID")]
  MissingResolutionDID,
  #[error("Missing Resolution Metadata")]
  MissingResolutionMetadata,
  #[error("Missing Resolution Document")]
  MissingResolutionDocument,
  #[error("Missing Resolution Document/Metadata")]
  MissingResolutionData,
  #[error("Invalid DID Resolution Query")]
  InvalidDIDQuery,
  #[error("Invalid DID Resolution Fragment")]
  InvalidDIDFragment,
  #[error("Invalid DID Resolution Service")]
  InvalidServiceProtocol,
}
