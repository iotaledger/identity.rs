// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SigningAlgorithm {
  Ed25519,
  Other(String),
}
