// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// Supported algorithms used to determine and potentially encrypt the content encryption key (CEK).
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CekAlgorithm {
  /// Uses the result of a Diffie-Hellman key exchange between a recipient's public key and an ephemeral secret as the
  /// content encryption key.
  ECDH_ES(AgreementInfo),
}

impl CekAlgorithm {
  /// Returns the JWE algorithm as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      CekAlgorithm::ECDH_ES(_agreement) => "ECDH-ES",
    }
  }
}

/// Agreement information used as the input for the concat KDF.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgreementInfo {
  /// Agreement PartyUInfo.
  apu: Vec<u8>,
  /// Agreement PartyVInfo.
  apv: Vec<u8>,
  /// SuppPubInfo.
  pub_info: Vec<u8>,
  /// SuppPrivInfo.
  priv_info: Vec<u8>,
}

impl AgreementInfo {
  /// Creates a new [`AgreementInfo`] instance.
  pub fn new(apu: Vec<u8>, apv: Vec<u8>, pub_info: Vec<u8>, priv_info: Vec<u8>) -> Self {
    Self {
      apu,
      apv,
      pub_info,
      priv_info,
    }
  }

  pub fn apu(&self) -> &[u8] {
    &self.apu
  }

  pub fn apv(&self) -> &[u8] {
    &self.apv
  }

  pub fn pub_info(&self) -> &[u8] {
    &self.pub_info
  }

  pub fn priv_info(&self) -> &[u8] {
    &self.priv_info
  }
}
