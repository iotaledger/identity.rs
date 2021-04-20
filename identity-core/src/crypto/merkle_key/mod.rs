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
pub use self::impls::Sha256;
pub use self::signer::MerkleSigner;
pub use self::signer::MerkleSigningKey;
pub use self::signer::SigningKey;
pub use self::tag::MerkleDigestTag;
pub use self::tag::MerkleSignatureTag;
pub use self::traits::MerkleDigest;
pub use self::traits::MerkleSignature;
pub use self::verifier::MerkleVerifier;
pub use self::verifier::VerificationKey;

#[cfg(test)]
mod tests;
