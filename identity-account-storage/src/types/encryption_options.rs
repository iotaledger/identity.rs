// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(non_camel_case_types)]

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
pub struct EncryptionOptions {
  encryption_algorithm: EncryptionAlgorithm,
  cek_algorithm: CEKAlgorithm,
}

impl EncryptionOptions {
  pub fn new(encryption_algorithm: EncryptionAlgorithm, cek_algorithm: CEKAlgorithm) -> Self {
    EncryptionOptions {
      encryption_algorithm,
      cek_algorithm,
    }
  }

  pub fn encryption_algorithm(&self) -> EncryptionAlgorithm {
    self.encryption_algorithm
  }

  pub fn cek_algorithm(&self) -> &CEKAlgorithm {
    &self.cek_algorithm
  }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EncryptionAlgorithm {
  Aes256Gcm,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CEKAlgorithm {
  ECDH_ES { agreement: AgreementInfo },
}

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
