// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::document_signer::DocumentSigner;
pub use self::document_verifier::DocumentVerifier;
pub use self::properties::VerifiableProperties;
pub use self::revocation::Revocation;
pub use self::verifier_options::VerifierOptions;

mod document_signer;
mod document_verifier;
mod properties;
mod revocation;
mod verifier_options;

#[cfg(test)]
mod tests;
