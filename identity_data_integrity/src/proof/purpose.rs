// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::convert::Infallible;
use std::str::FromStr;

use serde;
use serde::Deserialize;
use serde::Serialize;

/* 
/// Associates a purpose with a data integrity proof. 
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
*/

/// The purpose of a DataIntegrityProof. 
/// 
/// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes). 
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
//TODO: Optimize (de)serialization. 
pub struct ProofPurpose(Cow<'static, str>);
#[allow(non_upper_case_globals)]
impl ProofPurpose {
  /// Indicates that a given proof is only to be used for the purposes of an authentication protocol.
  /// 
  /// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes).  
  pub const AssertionMethod: Self = Self(Cow::Borrowed("assertionMethod")); 
  /// Indicates that a proof can only be used for making assertions, for example signing a Verifiable Credential.
  /// 
  /// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes). 
  pub const Authentication: Self = Self(Cow::Borrowed("authentication"));
  /// Indicates that a proof is used for for key agreement protocols, such as Elliptic Curve Diffie Hellman key agreement used by popular encryption libraries.
  /// 
  /// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes).  
  pub const KeyAgreement: Self = Self(Cow::Borrowed("keyAgreement"));
  /// Indicates that the proof can only be used for delegating capabilities.
  /// 
  /// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes). 
  pub const CapabilityDelegation: Self = Self(Cow::Borrowed("capabilityDelegation"));
  /// Indicates that the proof can only be used for invoking capabilities.
  /// 
  /// See the description in the [W3C DataIntegrityProof specification](https://w3c.github.io/vc-data-integrity/#proof-purposes). 
  pub const CapabilityInvocation: Self = Self(Cow::Borrowed("capabilityInvocation"));
}

impl FromStr for ProofPurpose {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let purpose = [
        Self::AssertionMethod,
        Self::Authentication,
        Self::KeyAgreement,
        Self::CapabilityDelegation,
        Self::CapabilityInvocation
      ].into_iter().find(|value| value.0 == s)
      .unwrap_or_else(|| Self(Cow::Owned(String::from(s))));
      Ok(purpose)
   }
}

impl From<String> for ProofPurpose {
  fn from(value: String) -> Self {
      Self(Cow::Owned(value))
  }
}

impl From<ProofPurpose> for String {
  fn from(value: ProofPurpose) -> Self {
      value.0.into()
  }
}