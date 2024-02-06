// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// This type represents errors that can occur when constructing credentials and presentations or their serializations.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// Caused by a failure to serialize or deserialize.
  #[error("serialization error: {0}")]
  SerializationError(&'static str, #[source] Option<identity_core::Error>),
  /// Caused by an invalid DID.
  #[error("invalid did")]
  DIDSyntaxError(#[source] identity_did::Error),
  /// Caused by an invalid DID document.
  #[error("invalid document")]
  InvalidDoc(#[source] identity_document::Error),
  #[cfg(feature = "iota-client")]
  /// Caused by a client failure during publishing.
  #[error("DID update: {0}")]
  DIDUpdateError(&'static str, #[source] Option<Box<iota_sdk::client::error::Error>>),
  #[cfg(feature = "iota-client")]
  /// Caused by a client failure during resolution.
  #[error("DID resolution failed")]
  DIDResolutionError(#[source] iota_sdk::client::error::Error),
  #[cfg(feature = "iota-client")]
  /// Caused by an error when building a basic output.
  #[error("basic output build error")]
  BasicOutputBuildError(#[source] iota_sdk::types::block::Error),
  /// Caused by an invalid network name.
  #[error("\"{0}\" is not a valid network name in the context of the `iota` did method")]
  InvalidNetworkName(String),
  #[cfg(feature = "iota-client")]
  /// Caused by a failure to retrieve the token supply.
  #[error("unable to obtain the token supply from the client")]
  TokenSupplyError(#[source] iota_sdk::client::Error),
  /// Caused by a mismatch of the DID's network and the network the client is connected with.
  #[error("unable to resolve a `{expected}` DID on network `{actual}`")]
  NetworkMismatch {
    /// The network of the DID.
    expected: String,
    /// The network the client is connected with.
    actual: String,
  },
  #[cfg(feature = "iota-client")]
  /// Caused by an error when fetching protocol parameters from a node.
  #[error("could not fetch protocol parameters")]
  ProtocolParametersError(#[source] iota_sdk::client::Error),
  /// Caused by an attempt to read state metadata that does not adhere to the IOTA DID method specification.
  #[error("invalid state metadata {0}")]
  InvalidStateMetadata(&'static str),
  #[cfg(feature = "revocation-bitmap")]
  /// Caused by a failure during (un)revocation of credentials.
  #[error("credential revocation error")]
  RevocationError(#[source] identity_credential::revocation::RevocationError),
  #[cfg(feature = "client")]
  /// Caused by an error when building an alias output.
  #[error("alias output build error")]
  AliasOutputBuildError(#[source] crate::block::Error),
  #[cfg(feature = "iota-client")]
  /// Caused by retrieving an output that is expected to be an alias output but is not.
  #[error("output with id `{0}` is not an alias output")]
  NotAnAliasOutput(iota_sdk::types::block::output::OutputId),
  /// Caused by an error when constructing an output id.
  #[error("conversion to an OutputId failed: {0}")]
  OutputIdConversionError(String),
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  /// Caused by an error in the Wasm bindings.
  #[error("JavaScript function threw an exception: {0}")]
  JsError(String),
  /// Caused by an error during JSON Web Signature verification.
  #[error("jws signature verification failed")]
  JwsVerificationError(#[source] identity_document::Error),
}
