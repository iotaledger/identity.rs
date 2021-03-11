// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Cryptographic Utilities

mod key;
mod proof;
mod signature;

pub mod merkle_key;
pub mod merkle_tree;

pub use self::key::KeyCollection;
pub use self::key::KeyPair;
pub use self::key::KeyRef;
pub use self::key::KeyType;
pub use self::key::PublicKey;
pub use self::key::SecretKey;
pub use self::proof::JcsEd25519;
pub use self::signature::Ed25519;
pub use self::signature::Named;
pub use self::signature::SetSignature;
pub use self::signature::Sign;
pub use self::signature::Signature;
pub use self::signature::SignatureValue;
pub use self::signature::Signer;
pub use self::signature::TrySignature;
pub use self::signature::TrySignatureMut;
pub use self::signature::Verifier;
pub use self::signature::Verify;
