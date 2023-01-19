// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Additional functionality for DID assisted digital signatures.

pub use self::document_signer::DocumentSigner;
pub use self::properties::VerifiableProperties;
pub use self::verifier_options::VerifierOptions;

mod document_signer;
mod properties;
mod verifier_options;

#[cfg(test)]
mod tests;
