// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::crypto::ProofPurpose;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataIntegrityProof {
  /// The proof type.
  #[serde(rename = "type")]
  pub(crate) type_: String,
  /// Purpose for which the proof was generated.
  #[serde(rename = "proofPurpose", skip_serializing_if = "Option::is_none")]
  pub proof_purpose: Option<ProofPurpose>,
  /// The verification method with which the proof was created.
  #[serde(rename = "verificationMethod")]
  pub(crate) verification_method: String,
  /// When the proof was generated.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<Timestamp>,
  /// Domain for which a proof is valid to mitigate replay attacks.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
  /// Challenge from a proof requester to mitigate replay attacks.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub challenge: Option<String>,
  /// A multibase-encoded string that contains the proof.
  #[serde(rename = "proofValue")]
  pub(crate) proof_value: String,
}

impl DataIntegrityProof {
  pub fn new() -> Self {
    todo!()
  }
}

// TODO: "The proof options MUST contain a type identifier for the cryptographic suite (type) and **any other properties
// needed by the cryptographic suite type**". Do we need to make this extensible somehow?
/// Holds attributes for a new [`Proof`](crate::crypto::Proof).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataIntegrityProofOptions {
  // The type identifier for the cryptographic suite (type).
  #[serde(rename = "type")]
  pub(crate) type_: String,
  /// The verification method with which the proof was created.
  #[serde(rename = "verificationMethod")]
  pub(crate) verification_method: String,
  /// [`Proof::created`](crate::crypto::Proof::created)
  pub(crate) created: Option<Timestamp>,
  /// [`Proof::challenge`](crate::crypto::Proof::challenge)
  pub(crate) challenge: Option<String>,
  /// [`Proof::domain`](crate::crypto::Proof::domain)
  pub(crate) domain: Option<String>,
}
