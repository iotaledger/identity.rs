// Copyright 2020-2022 IOTA Stiftung
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

  #[error("{0}")]
  InvalidDID(#[from] crate::did::DIDError),

  #[error("Verification Method Not Found")]
  MethodNotFound,

  /// Caused by invalid or missing properties when constructing a [`CoreDocument`].
  #[error("invalid document property: {0}")]
  InvalidDocument(&'static str, #[source] Option<::identity_core::Error>),

  #[error("Invalid Service Property: `id`")]
  BuilderInvalidServiceId,
  #[error("Invalid Service Property: `type`")]
  BuilderInvalidServiceType,
  #[error("Invalid Service Property: `service_endpoint`")]
  BuilderInvalidServiceEndpoint,

  // TODO: replace/merge these errors
  #[error("Invalid Verification Method Property: `id`")]
  BuilderInvalidMethodId,
  #[error("Invalid Verification Method Property: `controller`")]
  BuilderInvalidMethodController,
  #[error("Invalid Verification Method Property: `type`")]
  BuilderInvalidMethodType,
  #[error("Invalid Verification Method Property: `data`")]
  BuilderInvalidMethodData,

  #[error("invalid or empty verification method `id` fragment")]
  InvalidMethodFragment,
  #[error("Invalid Verification Method Type")]
  InvalidMethodType,
  /// Caused by attempting to add a verification method to a document, where a method with the same fragment already
  /// exists.
  #[error("verification method already exists")]
  MethodAlreadyExists,
  /// Caused by attempting to attach or detach a relationship on an embedded method.
  #[error("unable to modify relationships on embedded methods, use insert or remove instead")]
  InvalidMethodEmbedded,

  /// Caused by attempting to revoke an unsupported method.
  #[error("revocation is not supported for methods other than MerkleKeyCollection2021")]
  InvalidMethodRevocation,

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

  #[error("signature verification failed: {0}")]
  InvalidSignature(&'static str),

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
  InvalidResolutionService,
}
