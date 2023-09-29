// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "ed25519")]
mod ed25519_verifier;
mod eddsa_verifier;

#[cfg(feature = "ed25519")]
pub use ed25519_verifier::*;
pub use eddsa_verifier::*;
