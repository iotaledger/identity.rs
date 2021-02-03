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
pub use self::ed25519::MerkleKeyEd25519;
pub use self::ed25519::MerkleKeySignerEd25519;
pub use self::ed25519::MerkleKeyVerifierEd25519;
pub use self::signer::MerkleKeySigner;
pub use self::traits::MerkleKeyDigest;
pub use self::traits::MerkleKeyRevocation;
pub use self::traits::MerkleKeySignature;
pub use self::verifier::MerkleKeyVerifier;
