// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Decentralized Identifiers.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("{0}")]
  CoreError(#[from] ::identity_core::Error),
  #[error("{0}")]
  DIDError(#[from] ::did_url::Error),

  #[error("Duplicate Item in Ordered Set")]
  OrderedSetDuplicate,
  #[error("Verification Method Not Found")]
  QueryMethodNotFound,
  #[error("Signature Not Found")]
  QuerySignatureNotFound,

  #[error("Invalid Document Property: `id`")]
  InvalidDocumentId,

  #[error("Invalid Service Property: `id`")]
  InvalidServiceId,
  #[error("Invalid Service Property: `type`")]
  InvalidServiceType,
  #[error("Invalid Service Property: `service_endpoint`")]
  InvalidServiceEndpoint,

  #[error("Invalid Verification Method Property: `id`")]
  InvalidMethodId,
  #[error("Invalid Verification Method Fragment")]
  InvalidMethodIdFragment,
  #[error("Invalid Verification Method Property: `controller`")]
  InvalidMethodController,
  #[error("Invalid Verification Method Property: `type`")]
  InvalidMethodType,
  #[error("Invalid Verification Method Property: `data`")]
  InvalidMethodData,

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
