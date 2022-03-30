// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Cryptographic Utilities

pub use self::key::Ed25519;
pub use self::key::KeyPair;
pub use self::key::KeyType;
pub use self::key::PrivateKey;
pub use self::key::PublicKey;
pub use self::key::X25519;
pub use self::proof::JcsEd25519;
pub use self::proof::Proof;
pub use self::proof::ProofOptions;
pub use self::proof::ProofPurpose;
pub use self::proof::ProofValue;
pub use self::signature::Named;
pub use self::signature::SetSignature;
pub use self::signature::Sign;
pub use self::signature::Signer;
pub use self::signature::TrySignature;
pub use self::signature::TrySignatureMut;
pub use self::signature::Verifier;
pub use self::signature::Verify;

mod key;
mod proof;
mod signature;
