// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Additional functionality for DID assisted digital signatures.

pub use self::jws_verification_options::JwsVerificationOptions;
pub use self::properties::VerifiableProperties;
pub use self::verifier_options::VerifierOptions;

mod jws_verification_options;
mod properties;
mod verifier_options;
