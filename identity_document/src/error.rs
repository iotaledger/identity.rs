// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with DID Documents.

/// Alias for a [`Result`][::core::result::Result] with the error type [Error].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused by errors from the [identity_core] crate.
  #[error("core error")]
  CoreError(#[from] ::identity_core::Error),
  #[error("did error")]
  InvalidDID(#[from] identity_did::Error),

  #[error("verification method not found")]
  MethodNotFound,

  /// Caused by invalid or missing properties when constructing a [`CoreDocument`](crate::document::CoreDocument).
  #[error("invalid document property: {0}")]
  InvalidDocument(&'static str, #[source] Option<::identity_core::Error>),
  /// Caused by invalid or missing properties when constructing a [`Service`](crate::service::Service).
  #[error("invalid service property: {0}")]
  InvalidService(&'static str),
  /// Caused by invalid or missing properties when constructing a
  /// [`VerificationMethod`](::identity_verification::VerificationMethod).
  #[error("invalid verification method property: {0}")]
  InvalidMethod(&'static str),

  #[error("invalid or empty `id` fragment")]
  MissingIdFragment,
  #[error("Invalid Verification Method Type")]
  InvalidMethodType,
  /// Caused by attempting to add a verification method to a document, where a method or service with the same fragment
  /// already exists.
  #[error("unable to insert method: the id is already in use")]
  MethodInsertionError,
  /// Caused by attempting to attach or detach a relationship on an embedded method.
  #[error("unable to modify relationships on embedded methods, use insert or remove instead")]
  InvalidMethodEmbedded,

  /// Caused by attempting to insert a service whose id overlaps with a verification method or an already existing
  /// service.
  #[error("unable to insert service: the id is already in use")]
  InvalidServiceInsertion,

  #[error("unknown method scope")]
  UnknownMethodScope,
  #[error("unknown method type")]
  UnknownMethodType,

  #[error("invalid key data")]
  InvalidKeyData(#[source] identity_verification::Error),

  #[error("signature verification failed: {0}")]
  InvalidSignature(&'static str),

  #[error("unable to decode base64 string: `{0}`")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("revocation bitmap could not be deserialized or decompressed")]
  BitmapDecodingError(#[source] std::io::Error),
  #[error("revocation bitmap could not be serialized or compressed")]
  BitmapEncodingError(#[source] std::io::Error),
  #[error("jws verification failed")]
  JwsVerificationError(#[source] identity_verification::jose::error::Error),
}
