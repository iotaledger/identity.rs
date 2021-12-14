// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::document::DocumentSigner;
pub use self::document::DocumentVerifier;
pub use self::properties::VerifiableProperties;
pub use self::traits::Revocation;

mod document;
mod properties;
mod traits;

#[cfg(test)]
mod tests;
