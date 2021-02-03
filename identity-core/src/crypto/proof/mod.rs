// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for helping ensure the authenticity and integrity of
//! DID Documents and Verifiable Credentials.

mod jcsed25519signature2020;
mod merkle_key;

pub(crate) use self::jcsed25519signature2020::ed25519_sign;
pub(crate) use self::jcsed25519signature2020::ed25519_verify;
pub use self::jcsed25519signature2020::JcsEd25519Signature2020;
pub use self::merkle_key::MerkleKey;
pub use self::merkle_key::MerkleKeyDigest;
pub use self::merkle_key::MerkleKeyEd25519;
pub use self::merkle_key::MerkleKeyRevocation;
pub use self::merkle_key::MerkleKeySignature;
pub use self::merkle_key::MerkleKeySigner;
pub use self::merkle_key::MerkleKeySignerEd25519;
pub use self::merkle_key::MerkleKeyVerifier;
pub use self::merkle_key::MerkleKeyVerifierEd25519;
