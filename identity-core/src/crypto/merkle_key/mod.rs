// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An implementation of Merkle Key Collection Signatures.

mod base;
mod impls;
mod signer;
mod tag;
mod traits;
mod verifier;

pub use self::base::MerkleKey;
pub use self::impls::Blake2b256;
pub use self::impls::Ed25519;
pub use self::impls::Sha256;
pub use self::signer::Signer;
pub use self::signer::SigningKey;
pub use self::tag::MerkleTag;
pub use self::traits::MerkleDigest;
pub use self::traits::MerkleSignature;
pub use self::verifier::VerificationKey;
pub use self::verifier::Verifier;

#[cfg(test)]
mod tests;
