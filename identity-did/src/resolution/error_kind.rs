// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Types of errors that be returned from a [DID resolution][SPEC] process.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#dfn-did-resolution
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum ErrorKind {
  /// The DID supplied to the DID resolution function does not conform to
  /// valid syntax.
  #[serde(rename = "invalid-did")]
  InvalidDID,
  /// The DID resolver does not support the specified method.
  #[serde(rename = "not-supported")]
  NotSupported,
  /// The DID resolver was unable to return the DID document resulting from
  /// this resolution request.
  #[serde(rename = "not-found")]
  NotFound,
}
