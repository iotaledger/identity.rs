// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod public_key_multibase;

pub use public_key_multibase::PublicKeyMultibase;

#[non_exhaustive]
/// An enum of supported verification material formats.
///
/// Currently only [`PublicKeyMultibase`] is supported by this library, but it is
/// a goal to represent all formats listed in [the data integrity specification](https://w3c.github.io/vc-data-integrity/#verification-material).
pub enum VerificationMaterial {
  PublicKeyMultibase(PublicKeyMultibase),
}
