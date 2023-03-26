// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits related to signatures.

pub use self::core::Named;
pub use self::core::Sign;
pub use self::core::Signer;
pub use self::core::Verifier;
pub use self::core::Verify;
pub use self::data::GetSignature;
pub use self::data::GetSignatureMut;
pub use self::data::SetSignature;
pub use self::signature_options::JwsSignatureOptions; 
mod core;
mod data;
mod signature_options;
