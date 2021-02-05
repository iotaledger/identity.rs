// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for helping ensure the authenticity and integrity of
//! DID Documents and Verifiable Credentials.

mod jcsed25519signature2020;

pub(crate) use self::jcsed25519signature2020::ed25519_sign;
pub(crate) use self::jcsed25519signature2020::ed25519_verify;
pub use self::jcsed25519signature2020::JcsEd25519Signature2020;
