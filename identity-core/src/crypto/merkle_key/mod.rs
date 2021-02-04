// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An implementation of Merkle Key Collection Signatures.

mod base;
mod digest;
mod ed25519;
mod signer;
mod traits;
mod verifier;

pub use self::base::MerkleKey;
pub use self::ed25519::Ed25519;
pub use self::ed25519::SignerEd25519;
pub use self::ed25519::VerifierEd25519;
pub use self::signer::Signer;
pub use self::traits::Digest;
pub use self::traits::Revocation;
pub use self::traits::Signature;
pub use self::verifier::Verifier;
