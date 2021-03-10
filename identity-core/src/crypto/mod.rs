// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Cryptographic Utilities

mod ed25519;
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
pub use self::proof::JcsEd25519Signer;
pub use self::proof::JcsEd25519Verifier;
pub use self::signature::SetSignature;
pub use self::signature::Signature;
pub use self::signature::SignatureName;
pub use self::signature::SignatureSign;
pub use self::signature::SignatureValue;
pub use self::signature::SignatureVerify;
pub use self::signature::TrySignature;
pub use self::signature::TrySignatureMut;

pub(crate) use self::ed25519::ed25519_sign;
pub(crate) use self::ed25519::ed25519_verify;
