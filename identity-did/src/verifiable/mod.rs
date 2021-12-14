// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::document_signer::DocumentSigner;
pub use self::document_verifier::DocumentVerifier;
pub use self::revocation::Revocation;
pub use self::properties::VerifiableProperties;

mod document_signer;
mod document_verifier;
mod revocation;
mod properties;

#[cfg(test)]
mod tests;
