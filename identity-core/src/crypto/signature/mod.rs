// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod ed25519;
mod signature;
mod signature_value;
mod traits;

pub use self::ed25519::Ed25519;
pub use self::signature::Signature;
pub use self::signature_value::SignatureValue;
pub use self::traits::Named;
pub use self::traits::SetSignature;
pub use self::traits::Sign;
pub use self::traits::Signer;
pub use self::traits::TrySignature;
pub use self::traits::TrySignatureMut;
pub use self::traits::Verifier;
pub use self::traits::Verify;
