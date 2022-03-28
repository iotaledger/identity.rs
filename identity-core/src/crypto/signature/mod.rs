// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

pub use self::signature::Signature;
pub use self::signature_options::ProofPurpose;
pub use self::signature_options::SignatureOptions;
pub use self::signature_value::SignatureValue;
pub use self::traits::Named;
pub use self::traits::SetSignature;
pub use self::traits::Sign;
pub use self::traits::Signer;
pub use self::traits::TrySignature;
pub use self::traits::TrySignatureMut;
pub use self::traits::Verifier;
pub use self::traits::Verify;

mod signature;
mod signature_options;
mod signature_value;
mod traits;
