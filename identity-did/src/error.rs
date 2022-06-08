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
  /// Caused by invalid or missing properties when constructing a [`Service`].
  #[error("invalid service property: {0}")]
  InvalidService(&'static str),
  /// Caused by invalid or missing properties when constructing a [`VerificationMethod`].
  #[error("invalid verification method property: {0}")]
  InvalidMethod(&'static str),

  #[error("invalid or empty `id` fragment")]
  MissingIdFragment,
  #[error("Invalid Verification Method Type")]
  InvalidMethodType,
  /// Caused by attempting to add a verification method to a document, where a method with the same fragment already
  /// exists.
  #[error("verification method already exists")]
  MethodAlreadyExists,
  /// Caused by attempting to attach or detach a relationship on an embedded method.
  #[error("unable to modify relationships on embedded methods, use insert or remove instead")]
  InvalidMethodEmbedded,

  #[error("Unknown Method Scope")]
  UnknownMethodScope,
  #[error("Unknown Method Type")]
  UnknownMethodType,

  #[error("Invalid Key Data")]
  InvalidKeyData,
  #[error("Invalid Base58 Key Data")]
  InvalidKeyDataBase58,
  #[error("Invalid Multibase Key Data")]
  InvalidKeyDataMultibase,

  #[error("signature verification failed: {0}")]
  InvalidSignature(&'static str),

  #[error("unable to decode base64 string: `{0}`")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("revocation list could not be deserialized or decompressed")]
  BitmapDecodingError(#[source] std::io::Error),
  #[error("revocation list could not be serialized or compressed")]
  BitmapEncodingError(#[source] std::io::Error),
}
