// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::types::AgreementInfo;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CekAlgorithm {
  EcdhEs(AgreementInfo),
}

impl CekAlgorithm {
  /// Returns the JWE algorithm as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      CekAlgorithm::EcdhEs(_agreement) => "ECDH-ES",
    }
  }
}
