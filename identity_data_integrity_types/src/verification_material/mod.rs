// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod multikey;
mod public_key_multibase;

pub use multikey::*;

pub use public_key_multibase::PublicKeyMultibase;
use serde::Deserialize;
use serde::Serialize;

#[non_exhaustive]
/// An enum of supported verification material formats.
///
/// Currently only [`PublicKeyMultibase`] is supported by this library, but it is
/// a goal to represent all formats listed in [the data integrity specification](https://w3c.github.io/vc-data-integrity/#verification-material).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VerificationMaterial {
  PublicKeyMultibase(PublicKeyMultibase),
}
