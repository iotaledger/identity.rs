// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A ProofConfig according to https://w3c-ccg.github.io/di-eddsa-2020/#proof-configuration.
pub struct ProofConfig {
  type_: String,
  cryptosuite: Option<String>,
}