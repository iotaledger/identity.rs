// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde;
use serde::Deserialize;
use serde::Serialize;

/// Associates a purpose with a [`Proof`](crate::crypto::Proof).
///
/// See the [W3C Security Vocabulary description](https://w3c-ccg.github.io/security-vocab/#proofPurpose).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProofPurpose {
  /// Purpose is to assert a claim.
  /// See the [W3C Security Vocabulary description](https://www.w3.org/TR/did-core/#assertion).
  #[serde(rename = "assertionMethod")]
  AssertionMethod,
  /// Purpose is to authenticate the signer.
  /// See the [W3C Security Vocabulary description](https://www.w3.org/TR/did-core/#authentication).
  #[serde(rename = "authentication")]
  Authentication,
}
