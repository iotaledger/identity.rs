// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

//! Types and traits for helping ensure the authenticity and integrity of
//! DID Documents and Verifiable Credentials.

pub use self::ecdsa_secp256k1::EcdsaSecp256k1;
pub use self::jcs_ed25519::JcsEd25519;
pub use self::proof::Proof;
pub use self::proof_options::ProofOptions;
pub use self::proof_options::ProofPurpose;
pub use self::proof_value::ProofValue;

mod ecdsa_secp256k1;
mod jcs_ed25519;
mod proof;
mod proof_options;
mod proof_value;
